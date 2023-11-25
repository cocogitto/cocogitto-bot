use axum::routing::post;
use axum::{Json, Router, ServiceExt};
use axum_macros::debug_handler;
use std::net::SocketAddr;

use cocogitto::settings::Settings as CogSettings;
use model::github_event::pull_request_event::PullRequestEvent;
use model::report::CommitReport;
use model::Commit;
use octo::authenticate;
use octo::commits::GetCommits;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::model::github_event::pull_request_event::PullRequestAction;
use crate::octo::check_run::CheckRunSummary;
use crate::settings::Settings;

mod comment;
mod error;
mod event_guard;
mod model;
mod octo;
mod settings;

// #[post("/", data = "<body>", rank = 2, format = "application/json")]
async fn pull_request(_event: (), body: Json<PullRequestEvent>) -> &'static str {
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

    // Get the comments for the current pull request
    let issues = octo
        .issues(owner, repo)
        .list_comments(*pull_request_number)
        .page(1u32)
        .send()
        .await
        .unwrap();

    // Try to find a previous cocogitto-bot comment
    let previous_comment = issues
        .items
        .iter()
        .find(|comment| comment.user.login == "cocogitto-bot[bot]");

    // Delete this comment if found
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

    // Get all commits for the current pull request
    let commits = octo
        .get_commits(owner, repo, *pull_request_number)
        .await
        .unwrap();

    // Check the target repo for an existing Cocogitto config file
    let cog_file = octo
        .repos(owner, repo)
        .get_content()
        .path("cog.toml")
        .r#ref(&event.repository.default_branch)
        .send()
        .await
        .ok()
        .and_then(|mut content| content.take_items().into_iter().next())
        .and_then(|cog| cog.decoded_content())
        .unwrap_or("".to_string());

    // Parse the config file into Cocogitto `Settings` (falling
    // back to the default if the target repo doesn't have a `cog.toml`)
    let cog_config = CogSettings::try_from(cog_file).unwrap_or_else(|_| CogSettings {
        ignore_merge_commits: true,
        ..CogSettings::default()
    });

    // Turn them into conventional commits report
    let reports: Vec<CommitReport> = commits
        .iter()
        .map(Commit::from)
        .map(|commit| CommitReport::from_commit(commit, cog_config.ignore_merge_commits))
        .collect();

    // Send a github check-run for every single commit in the R
    let outcome = octo::check_run::per_commit_check_run(&octo, owner, repo, &commits)
        .await
        .unwrap();

    info!(
        "Commit statuses checked in {}/{}#{}",
        owner, repo, pull_request_number
    );

    // Build check-run summary comment (failure if a single commit fails)
    let comment = match outcome {
        CheckRunSummary::Errored => comment::build_comment_failure(reports),
        CheckRunSummary::NoError => comment::build_comment_success(reports),
    };

    // Send the comment
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Settings::get()?;

    let router = Router::new()
        .route("/", post(pull_request_handler))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&config.address())
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

#[debug_handler]
async fn pull_request_handler(dum: String) -> () {}
