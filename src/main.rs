#![feature(rustc_private)]
#[macro_use]
extern crate rocket;

use crate::commit_event::CommitEvent;
use rocket::serde::json::Json;
use crate::pull_request_event::PullRequest;
use crate::event_guard::{CommitEventType, PullRequestEventType};
use conventional_commit_parser::parse;
use conventional_commit_parser::commit::ConventionalCommit;
use octocrab::models::events::Event;
use octocrab::models::events::payload::{CommitCommentEventPayload, PushEventPayload};
use conventional_commit_parser::error::ParseError;
use octocrab::Octocrab;

mod commit_event;
pub mod error;
mod pull_request_event;
mod event_guard;
mod authenticate;

#[tokio::main]
pub fn main() {
    let octocrab = Octocrab::builder()
        .personal_token()
        .build()?;

}
// #[post("/", data = "<body>", format = "application/json")]
// async fn commit(_event: CommitEventType, body: Json<CommitEvent>) -> &'static str {
//     let messages: Vec<ParseError> = body.into_inner()
//         .commits
//         .into_iter()
//         .map(|commit| commit.message)
//         .map(|message| parse(&message))
//         .filter_map(Result::err)
//         .collect();
//
//     "ok"
// }
//
// #[post("/", data = "<body>", rank = 2, format = "application/json")]
// async fn pull_request(_event: PullRequestEventType, body: Json<PullRequest>) -> &'static str {
//     println!("{:?}", body.0);
//     "ok"
// }
//
// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![commit, pull_request])
// }