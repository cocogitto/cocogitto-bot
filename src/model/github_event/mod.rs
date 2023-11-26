use serde::Deserialize;

pub mod pull_request_event;

#[derive(Debug, Deserialize)]
pub struct CheckSuiteEvent {
    pub action: CheckSuiteAction,
    pub check_suite: CheckSuitePayload,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CheckSuiteAction {
    Requested,
    ReRequested,
    Completed,
}
#[derive(Debug, Deserialize)]
pub struct CheckSuitePayload {
    pub repository: Repository,
    pub pull_request: Vec<PullRequest>,
    pub installation: Installation,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub name: String,
    pub owner: RepositoryOwner,
    pub default_branch: String,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub number: u64,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryOwner {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct Installation {
    pub id: u64,
}
