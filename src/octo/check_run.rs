use octocrab::{models, Octocrab};
use rocket::serde::{Deserialize, Serialize};

use crate::model::report::CommitReport;
use crate::model::Commit;
use crate::octo::commits::CommitObjectDto;

#[async_trait::async_trait]
pub trait CheckRunExt {
    async fn check_run(
        &self,
        owner: &str,
        repo: &str,
        check_run: &CheckRunResult,
    ) -> octocrab::Result<models::CheckRun>;
}

#[async_trait::async_trait]
impl CheckRunExt for Octocrab {
    async fn check_run(
        &self,
        owner: &str,
        repo: &str,
        check_run: &CheckRunResult,
    ) -> octocrab::Result<models::CheckRun> {
        let url = format!("/repos/{}/{}/check-runs", owner, repo);
        self.post(url, Some(check_run)).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CheckRunResult {
    pub output: CheckOutput,
    pub name: String,
    pub head_sha: String,
    pub conclusion: CheckRunConclusion,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CheckOutput {
    pub title: String,
    pub summary: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "lowercase")]
pub enum CheckRunConclusion {
    Success,
    Failure,
}

pub enum CheckRunSummary {
    Errored,
    NoError,
}

pub async fn per_commit_check_run(
    octo: &Octocrab,
    owner: &str,
    repo: &str,
    commits: &[CommitObjectDto],
) -> octocrab::Result<CheckRunSummary> {
    let mut reports: Vec<CommitReport> = commits
        .iter()
        .map(Commit::from)
        .map(CommitReport::from)
        .collect();

    let has_failures = reports
        .iter()
        .any(|report| matches!(report, CommitReport::Error(_)));

    let previous_commits_reports: Vec<CommitReport> = reports.drain(0..reports.len() - 1).collect();

    for report in previous_commits_reports {
        let check_run = CheckRunResult::from(report);
        octo.check_run(owner, repo, &check_run).await?;
    }

    if has_failures {
        // Only one report remains
        let head = reports.get(0).unwrap();

        let text = match head {
            CommitReport::Success(_) => {
                "Found non-compliant commits in the current pull request :\n".to_string()
            }
            CommitReport::Error(err) => err.to_string(),
        };

        let final_run = CheckRunResult {
            output: CheckOutput {
                title: "Conventional commits check".to_string(),
                summary: "Failure".to_string(),
                text,
            },
            name: "Cog status check".to_string(),
            head_sha: head.get_sha().to_string(),
            conclusion: CheckRunConclusion::Failure,
        };

        octo.check_run(owner, repo, &final_run).await?;
        Ok(CheckRunSummary::Errored)
    } else {
        Ok(CheckRunSummary::NoError)
    }
}

impl From<CommitReport> for CheckRunResult {
    fn from(report: CommitReport) -> Self {
        match report {
            CommitReport::Success(commit) => CheckRunResult {
                output: CheckOutput {
                    title: "Conventional commits check".to_string(),
                    summary: "Success".to_string(),
                    text: "".to_string(),
                },
                name: "Cog status check".to_string(),
                head_sha: commit.sha,
                conclusion: CheckRunConclusion::Success,
            },
            CommitReport::Error(report) => CheckRunResult {
                output: CheckOutput {
                    title: "Conventional commits check".to_string(),
                    summary: "Failure".to_string(),
                    text: report.to_string(),
                },
                name: "Cog status check".to_string(),
                head_sha: report.sha,
                conclusion: CheckRunConclusion::Failure,
            },
        }
    }
}
