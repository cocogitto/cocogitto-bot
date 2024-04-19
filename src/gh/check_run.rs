use octocrab::models::checks::CheckRun;
use octocrab::params::checks::CheckRunStatus;
use serde::Serialize;
use tracing::info;

use crate::gh::CocogittoBot;

#[derive(Debug, Serialize)]
pub struct CheckOutput {
    pub title: String,
    pub summary: String,
    pub text: String,
}

const CHECK_RUN_NAME: &str = "cog-status-check";

impl CocogittoBot {
    pub async fn create_check_runs(&self) -> octocrab::Result<CheckRun> {
        match self.pull_request_number {
            None => info!("Creating check runs for {}/{}", self.owner, self.repo),
            Some(number) => info!(
                "Creating check runs for {}/{}#{}",
                self.owner, self.repo, number
            ),
        }

        self.inner
            .checks(&self.owner, &self.repo)
            .create_check_run(CHECK_RUN_NAME, &self.head_sha)
            .status(CheckRunStatus::Queued)
            .send()
            .await
    }
}
