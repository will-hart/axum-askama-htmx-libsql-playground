use std::net::SocketAddr;

use axum::{
    extract::{MatchedPath, Request},
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod state;
mod webroutes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axumtest=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(webroutes::home))
        .route("/increment", post(state::increment))
        .route("/reset", post(state::reset))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .with_state(state::AppState::default());

    let addr: SocketAddr = "0.0.0.0:8011".parse().unwrap();
    let listener = TcpListener::bind(addr).await?;
    info!("Starting server on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}
