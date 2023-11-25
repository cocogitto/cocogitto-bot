use serde::Deserialize;

use crate::model::installation::Installation;

#[derive(Debug, Deserialize)]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    pub number: u64,
    pub repository: PullRequestRepository,
    pub installation: Installation,
    pub pull_request: PullRequestEventInner,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestEventInner {
    pub head: Head,
}

#[derive(Debug, Deserialize)]
pub struct Head {
    pub sha: String,
}

#[derive(Debug, Deserialize, Eq, PartialOrd, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestAction {
    Synchronize,
    Opened,
    Closed,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestRepository {
    pub name: String,
    pub owner: PullRequestOwner,
    pub default_branch: String,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestOwner {
    pub login: String,
}
