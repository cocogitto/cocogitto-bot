use axum::extract::State;
use axum::http::HeaderMap;

use axum::routing::post;
use axum::{Json, Router, ServiceExt};
use axum_macros::debug_handler;

use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::error::AppResult;

use crate::model::github_event::{CheckSuiteAction, CheckSuiteEvent};
use crate::octo::CocogittoBot;
use crate::settings::Settings;

mod comment;
mod error;
mod model;
mod octo;
mod settings;

#[derive(Clone)]
pub struct AppState {
    github_key: String,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "cocogitto_github_app=info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Settings::get()?;
    let addr = config.address();

    let router = Router::new()
        .route("/", post(pull_request_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState {
            github_key: config.github_private_key,
        });

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

#[debug_handler]
async fn pull_request_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(event): Json<CheckSuiteEvent>,
) -> AppResult<()> {
    let Some(event_header) = headers.get("X-Github-Event") else {
        warn!("'X-Github-Event' header missing, ignoring request");
        return Ok(());
    };

    let Ok("check_suite") = event_header.to_str() else {
        info!("Ignoring non check_suite event");
        return Ok(());
    };

    if event.action == CheckSuiteAction::Completed {
        info!("Ignoring completed check_suite");
        return Ok(());
    }

    if event.check_suite.pull_request.is_empty() {
        info!("Ignoring check_suite with no pull request");
    }

    CocogittoBot::from_check_suite(event.check_suite, &state.github_key)
        .await?
        .run()
        .await?;

    Ok(())
}
