use cocogitto::settings::Settings as CogSettings;
use octocrab::checks::CheckRunStatus;
use octocrab::models::checks::CheckRun;
use octocrab::models::issues::Comment;
use octocrab::models::repos::RepoCommit;
use octocrab::Octocrab;
use tokio::join;
use tracing::{info, warn};

use event::CheckSuiteEvent;

use crate::cog::report::CogBotReportBuilder;
use crate::gh::authenticate::authenticate;
use crate::gh::check_run::CheckOutput;
use crate::gh::commits::GetCommits;

pub mod authenticate;
pub mod check_run;
pub mod commits;
pub mod event;

pub struct CocogittoBot {
    inner: Octocrab,a
    owner: String,
    repo: String,
    head_sha: String,
    pull_request_number: u64,
    default_branch: String,
}

const COCOGITTO_BOT_LOGIN: &str = "cocogitto-bot[bot]";

impl CocogittoBot {
    pub async fn from_check_suite(event: CheckSuiteEvent, gh_key: &str) -> octocrab::Result<Self> {
        let check_suite = event.check_suite;
        let installation = event.installation;
        let repository = event.repository;

        if check_suite.pull_requests.len() > 1 {
            warn!("Multiple pull request check_suite event will handle only the first PR");
        }

        let inner = authenticate(installation.id, &repository.name, gh_key).await?;
        let pull_request = check_suite
            .pull_requests
            .into_iter()
            .next()
            .expect("Pull request should not be empty");

        Ok(Self {
            inner,
            owner: repository.owner.login,
            repo: repository.name,
            head_sha: check_suite.head_sha,
            pull_request_number: pull_request.number,
            default_branch: repository.default_branch,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        self.delete_previous_comment_if_exists().await?;
        let cog_config = self.get_cog_config().await?;
        let commits = self.get_pull_request_commits().await?;
        let check_run = self.create_check_runs().await?;
        self.build_and_send_commit_reports(commits, cog_config, check_run)
            .await
    }

    async fn get_pull_request_commits(&self) -> octocrab::Result<Vec<RepoCommit>> {
        self.inner
            .get_commits(&self.owner, &self.repo, self.pull_request_number)
            .await
    }

    async fn get_cog_config(&self) -> octocrab::Result<CogSettings> {
        let cog_file = self
            .inner
            .repos(&self.owner, &self.repo)
            .get_content()
            .path("cog.toml")
            .r#ref(&self.default_branch)
            .send()
            .await
            .ok()
            .and_then(|mut content| content.take_items().into_iter().next())
            .and_then(|cog| cog.decoded_content())
            .unwrap_or("".to_string());

        Ok(
            CogSettings::try_from(cog_file).unwrap_or_else(|_| CogSettings {
                ignore_merge_commits: true,
                ..CogSettings::default()
            }),
        )
    }

    async fn delete_previous_comment_if_exists(&self) -> octocrab::Result<()> {
        let issues = self
            .inner
            .issues(&self.owner, &self.repo)
            .list_comments(self.pull_request_number)
            .page(1u32)
            .send()
            .await?;

        let previous_comment = issues
            .items
            .iter()
            .find(|comment| comment.user.login == COCOGITTO_BOT_LOGIN);

        if let Some(previous_comment) = previous_comment {
            info!(
                "Deleting comment {} in {}/{}#{}",
                previous_comment.id, self.owner, self.repo, self.pull_request_number
            );

            self.inner
                .issues(&self.owner, &self.repo)
                .delete_comment(previous_comment.id)
                .await?;
        }

        Ok(())
    }

    async fn build_and_send_commit_reports(
        &self,
        commits: Vec<RepoCommit>,
        cog_config: CogSettings,
        check_run: CheckRun,
    ) -> anyhow::Result<()> {
        let report = CogBotReportBuilder::new(&commits, cog_config);

        let comment = if report.has_error() {
            report.build_comment_failure()
        } else {
            report.build_comment_success()
        };

        let summary = if report.has_error() {
            "failure".to_string()
        } else {
            "success".to_string()
        };

        let check_output = CheckOutput {
            title: format!("Cog status check #{}", self.pull_request_number),
            summary: summary.clone(),
            text: comment.clone(),
        };

        let issue_handler = self.inner.issues(&self.owner, &self.repo);
        let check_handler = self.inner.checks(&self.owner, &self.repo);

        let (checks, comment) = join!(
            check_handler
                .update_check_run(check_run.id)
                .conclusion(&summary)
                .output(check_output.to_value())
                .status(CheckRunStatus::Completed)
                .send(),
            issue_handler.create_comment(self.pull_request_number, &comment)
        );

        let _: CheckRun = checks?;
        let _: Comment = comment?;

        Ok(())
    }

    async fn create_check_runs(&self) -> octocrab::Result<CheckRun> {
        info!(
            "Creating check runs for {}/{}#{}",
            self.owner, self.repo, self.pull_request_number
        );

        self.inner
            .checks(&self.owner, &self.repo)
            .create_check_run("Cog status check", &self.head_sha)
            .status(CheckRunStatus::InProgress)
            .send()
            .await
    }
}
