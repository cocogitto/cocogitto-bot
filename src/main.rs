use autometrics::prometheus_exporter::PrometheusResponse;
use autometrics::{autometrics, prometheus_exporter};
use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_macros::debug_handler;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use gh::event::{CheckSuiteAction, CheckSuiteEvent};

use crate::error::AppResult;
use crate::gh::CocogittoBot;
use crate::settings::Settings;

mod cog;
mod error;
mod gh;
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
                .unwrap_or_else(|_| "cocogitto_github_app=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing prometheus exporter");
    prometheus_exporter::init();

    let config = Settings::get()?;
    info!("GH_KEY: \n{}", config.github_private_key);

    let addr = config.address();

    let router = Router::new()
        .route("/", post(pull_request_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState {
            github_key: config.github_private_key,
        })
        .route("/health", get(get_health))
        .route("/metrics", get(get_metrics));

    info!("Serving cocogitto bot at {}", &addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

#[debug_handler]
#[autometrics]
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

    CocogittoBot::from_check_suite(event, &state.github_key)
        .await?
        .run()
        .await?;

    Ok(())
}

#[debug_handler]
pub async fn get_metrics() -> PrometheusResponse {
    prometheus_exporter::encode_http_response()
}

#[debug_handler]
pub async fn get_health() -> AppResult<()> {
    Ok(())
}
