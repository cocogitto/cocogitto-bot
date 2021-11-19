#![feature(rustc_private)]
#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

use model::commit_event::CommitEvent;
use model::pull_request_event::PullRequest;

use crate::event_guard::{CommitEventType, PullRequestEventType};

pub mod model;
pub mod error;
mod event_guard;
mod authenticate;
mod comment;

#[post("/", data = "<body>", format = "application/json")]
async fn commit(_event: CommitEventType, body: Json<CommitEvent>) -> &'static str {
    let commit_event = body.into_inner();
    println!("{:?}", commit_event);
    let conventional_commit_errors = commit_event.extract_errors();

    if !conventional_commit_errors.is_empty() {
        let owner = &commit_event.repository.owner.name;
        let repo = &commit_event.repository.name;
        let installation_id = commit_event.installation.id;

        let octo = authenticate::authenticate(installation_id, &repo)
            .await
            .expect("Unable to authenticate");


        let comment: String = conventional_commit_errors.iter()
            .map(|report| report.to_string())
            .collect::<Vec<String>>()
            .join("\n");


        let result = octo.issues(owner, repo)
            .create_comment(1, comment).await;
        println!("{:?}", result);
    }

    "ok"
}
//https://api.github.com/repos/octocat/hello-world/pulls/42/comments
//https://api.github.com/repos/cocogitto/cocogitto_bot_playground/pulls/1/comments

#[post("/", data = "<body>", rank = 2, format = "application/json")]
async fn pull_request(_event: PullRequestEventType, body: Json<PullRequest>) -> &'static str {
    println!("{:?}", body.0);
    "ok"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![commit, pull_request])
}