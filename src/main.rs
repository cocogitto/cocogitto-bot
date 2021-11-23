#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

use rocket::serde::json::Json;

use model::github_event::pull_request_event::PullRequestEvent;
use model::report::CommitReport;
use model::Commit;
use octo::authenticate;
use octo::commits::GetCommits;

use crate::event_guard::PullRequestEventType;
use crate::model::github_event::pull_request_event::PullRequestAction;
use crate::octo::check_run::CheckRunSummary;

mod comment;
mod error;
mod event_guard;
mod model;
mod octo;

#[post("/", data = "<body>", rank = 2, format = "application/json")]
async fn pull_request(_event: PullRequestEventType, body: Json<PullRequestEvent>) -> &'static str {
    let event = body.0;

    if event.action == PullRequestAction::Closed {
        return "ok";
    };

    let owner = &event.repository.owner.login;
    let repo = &event.repository.name;
    let pull_request_number = &event.number;
    let installation_id = event.installation.id;

    let octo = authenticate::authenticate(installation_id, repo)
        .await
        .expect("Unable to authenticate");

    let issues = octo
        .issues(owner, repo)
        .list_comments(*pull_request_number)
        .page(1u32)
        .send()
        .await
        .unwrap();

    let previous_comment = issues
        .items
        .iter()
        .find(|comment| comment.user.login == "cocogitto-bot[bot]");
    if let Some(previous_comment) = previous_comment {
        info!(
            "Deleting comment {} in {}/{}#{}",
            previous_comment.id, owner, repo, pull_request_number
        );
        octo.issues(owner, repo)
            .delete_comment(previous_comment.id)
            .await
            .unwrap();
    }

    let commits = octo
        .get_commits(owner, repo, *pull_request_number)
        .await
        .unwrap();

    let reports: Vec<CommitReport> = commits
        .iter()
        .map(Commit::from)
        .map(Commit::into_report)
        .collect();

    let outcome = octo::check_run::per_commit_check_run(&octo, owner, repo, &commits)
        .await
        .unwrap();
    info!(
        "Commit statuses checked in {}/{}#{}",
        owner, repo, pull_request_number
    );
    let comment = match outcome {
        CheckRunSummary::Errored => comment::build_comment_failure(reports),
        CheckRunSummary::NoError => comment::build_comment_success(reports),
    };

    octo.issues(owner, repo)
        .create_comment(*pull_request_number, &comment)
        .await
        .unwrap();
    info!(
        "Comment summary sent to {}/{}#{}",
        owner, repo, pull_request_number
    );

    "ok"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![pull_request])
}
