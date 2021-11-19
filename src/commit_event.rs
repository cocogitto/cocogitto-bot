use chrono::{NaiveDate, NaiveDateTime};
use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CommitEvent {
    pub commits: Vec<Commit>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Commit {
    pub id: String,
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub url: String,
    pub author: Committer,
    pub committer: Committer,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Committer {
    name: String,
    email: String,
    username: String,
}