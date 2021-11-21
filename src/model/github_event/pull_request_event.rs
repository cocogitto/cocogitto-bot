use rocket::serde::Deserialize;

use crate::model::installation::Installation;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    pub number: u64,
    pub repository: PullRequestRepository,
    pub installation: Installation,
    pub pull_request: PullRequestEventInner,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PullRequestEventInner {
    pub head: Head,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Head {
    pub sha: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "lowercase")]
pub enum PullRequestAction {
    Synchronize,
    Opened,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PullRequestRepository {
    pub name: String,
    pub owner: PullRequestOwner,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PullRequestOwner {
    pub login: String,
}
