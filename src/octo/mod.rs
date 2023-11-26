use cocogitto::settings::Settings as CogSettings;
use octocrab::Octocrab;
use tracing::{info, warn};

use crate::comment::{build_comment_failure, build_comment_success};

use crate::model::github_event::CheckSuitePayload;
use crate::model::report::CommitReport;
use crate::model::Commit;
use crate::octo::authenticate::authenticate;
use crate::octo::check_run::CheckRunSummary;
use crate::octo::commits::{CommitObjectDto, GetCommits};

pub mod authenticate;
pub mod check_run;
pub mod commits;

pub struct CocogittoBot {
    inner: Octocrab,
    owner: String,
    repo: String,
    pull_request_number: u64,
    default_branch: String,
    installation_id: u64,
}

const COCOGITTO_BOT_LOGIN: &str = "cocogitto-bot[bot]";

impl CocogittoBot {
    pub async fn from_check_suite(
        payload: CheckSuitePayload,
        gh_key: &str,
    ) -> octocrab::Result<Self> {
        if payload.pull_request.len() > 1 {
            warn!("Multiple pull request check_suite event will handle only the first PR");
        }

        let inner = authenticate(payload.installation.id, &payload.repository.name, gh_key).await?;
        let pull_request = payload
            .pull_request
            .into_iter()
            .next()
            .expect("Pull request should not be empty");

        Ok(Self {
            inner,
            owner: payload.repository.owner.login,
            repo: payload.repository.name,
            pull_request_number: pull_request.number,
            default_branch: payload.repository.default_branch,
            installation_id: payload.installation.id,
        })
    }

    pub async fn run(&self) -> octocrab::Result<()> {
        self.delete_previous_comment_if_exists().await?;
        let cog_config = self.get_cog_config().await?;
        let commits = self.get_pull_request_commits().await?;
        self.build_and_send_commit_reports(commits, cog_config)
            .await
    }
    async fn get_pull_request_commits(&self) -> octocrab::Result<Vec<CommitObjectDto>> {
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
        commits: Vec<CommitObjectDto>,
        cog_config: CogSettings,
    ) -> octocrab::Result<()> {
        let reports: Vec<CommitReport> = commits
            .iter()
            .map(Commit::from)
            .map(|commit| CommitReport::from_commit(commit, cog_config.ignore_merge_commits))
            .collect();

        // Send a github check-run for every single commit in the R
        let outcome =
            check_run::per_commit_check_run(&self.inner, &self.owner, &self.repo, &commits).await?;

        let comment = match outcome {
            CheckRunSummary::Errored => build_comment_failure(reports),
            CheckRunSummary::NoError => build_comment_success(reports),
        };

        self.inner
            .issues(&self.owner, &self.repo)
            .create_comment(self.pull_request_number, &comment)
            .await?;

        Ok(())
    }
}
