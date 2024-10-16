use anyhow::anyhow;
use cocogitto::settings::Settings as CogSettings;
use octocrab::models::checks::CheckRun;
use octocrab::models::issues::Comment;
use octocrab::models::repos::RepoCommit;
use octocrab::params::checks::{CheckRunConclusion, CheckRunOutput, CheckRunStatus};
use octocrab::Octocrab;
use tokio::join;
use tracing::{error, info, warn};

use event::CheckSuiteEvent;

use crate::cog::report::CogBotReportBuilder;
use crate::gh::authenticate::authenticate;
use crate::gh::commits::GetCommits;
use crate::gh::event::PullRequestEvent;

pub mod authenticate;
pub mod check_run;
pub mod commits;
pub mod event;

pub struct CocogittoBot {
    inner: Octocrab,
    owner: String,
    repo: String,
    head_sha: String,
    pull_request_number: Option<u64>,
    default_branch: String,
}

const COCOGITTO_BOT_LOGIN: &str = "cocogitto-bot[bot]";

impl CocogittoBot {
    pub async fn from_check_suite(event: CheckSuiteEvent, gh_key: &str) -> anyhow::Result<Self> {
        let check_suite = event.check_suite;
        let installation = event.installation;
        let repository = event.repository;

        if check_suite.pull_requests.len() > 1 {
            warn!("Multiple pull request check_suite event will handle only the first PR");
        }

        info!("Authenticating to github api");
        let auth = authenticate(installation.id, &repository.name, gh_key).await;
        if let Err(auth_error) = &auth {
            error!("Failed to authenticate: {auth_error}");
        }

        let inner = auth?;
        let pull_request_number = check_suite
            .pull_requests
            .into_iter()
            .next()
            .map(|pr| pr.number);

        Ok(Self {
            inner,
            owner: repository.owner.login,
            repo: repository.name,
            head_sha: check_suite.head_sha,
            pull_request_number,
            default_branch: repository.default_branch,
        })
    }

    pub async fn from_pull_request(event: PullRequestEvent, gh_key: &str) -> anyhow::Result<Self> {
        let installation = event.installation;
        let repository = event.repository;

        info!("Authenticating to github api");
        let auth = authenticate(installation.id, &repository.name, gh_key).await;
        if let Err(auth_error) = &auth {
            return Err(anyhow!("Failed to authenticate: {auth_error}"));
        }

        let inner = auth?;
        let pull_request_number = Some(event.inner.pull_request.number);

        let Some(default_branch) = repository.default_branch else {
            return Err(anyhow!("default_branch missing from pull_request event"));
        };

        let Some(owner) = repository.owner.map(|owner| owner.login) else {
            return Err(anyhow!("owner missing from pull_request event"));
        };

        Ok(Self {
            inner,
            owner,
            repo: repository.name,
            head_sha: event.inner.pull_request.head.sha,
            pull_request_number,
            default_branch,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let check_run = self.create_check_runs().await?;

        if self.pull_request_number.is_none() {
            let number = check_run
                .pull_requests
                .first()
                .map(|pr| pr.number)
                .expect("Check run PR should be known at this point");

            self.pull_request_number = Some(number);
        }

        self.inner
            .checks(&self.owner, &self.repo)
            .update_check_run(check_run.id)
            .status(CheckRunStatus::InProgress)
            .send()
            .await?;

        self.delete_previous_comment_if_exists().await?;

        let cog_config = self.get_cog_config().await?;
        let commits = self.get_pull_request_commits().await?;

        self.build_and_send_commit_reports(commits, cog_config, check_run)
            .await?;
        Ok(())
    }

    async fn get_pull_request_commits(&self) -> octocrab::Result<Vec<RepoCommit>> {
        self.inner
            .get_commits(&self.owner, &self.repo, self.pull_request_number.unwrap())
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
        let pull_request_number = self
            .pull_request_number
            .expect("pull_request_number should be set");
        let issues = self
            .inner
            .issues(&self.owner, &self.repo)
            .list_comments(pull_request_number)
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
                previous_comment.id, self.owner, self.repo, pull_request_number
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

        let (summary, conclusion) = if report.has_error() {
            ("failure".to_string(), CheckRunConclusion::Failure)
        } else {
            ("success".to_string(), CheckRunConclusion::Success)
        };

        let check_output = CheckRunOutput {
            title: format!("Cog status check #{}", self.pull_request_number.unwrap()),
            summary: summary.clone(),
            text: Some(comment.clone()),
            annotations: vec![],
            images: vec![],
        };

        let issue_handler = self.inner.issues(&self.owner, &self.repo);
        let check_handler = self.inner.checks(&self.owner, &self.repo);

        let (checks, comment) = join!(
            check_handler
                .update_check_run(check_run.id)
                .conclusion(conclusion)
                .output(check_output)
                .status(CheckRunStatus::Completed)
                .send(),
            issue_handler.create_comment(self.pull_request_number.unwrap(), &comment)
        );

        let _: CheckRun = checks?;
        let _: Comment = comment?;

        Ok(())
    }
}
