use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{MatchedPath, Request, State},
    routing::{get, post},
    Router,
};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Default)]
struct AppState {
    pub counter: Arc<Mutex<u16>>,
}

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
        .route("/", get(home))
        .route("/increment", post(increment))
        .route("/reset", post(reset))
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
        .with_state(AppState::default());

    let addr: SocketAddr = "0.0.0.0:8011".parse().unwrap();
    let listener = TcpListener::bind(addr).await?;
    info!("Starting server on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;

    Ok(())
}

async fn home() -> Markup {
    html!(
        (DOCTYPE)
        html {
            head {
                (PreEscaped("<script src=\"https://unpkg.com/htmx.org@1.9.10\" crossorigin=\"anonymous\"></script>"))
                (PreEscaped("<style>.leftpad { margin-left: 1em; }</style>"))
                title { "Fred" }
            }
            body {
                .intro {
                    span { "Counter " }
                    span #targetme { "0" }

                    button.leftpad hx-post="/increment" hx-target="#targetme" hx-swap="innerHTML" {
                        "Increment"
                    }

                    button.leftpad hx-post="/reset" hx-target="#targetme" hx-swap="innerHTML" {
                        "Reset"
                    }
                }
            }
        }
    )
}

async fn increment(state: State<AppState>) -> Markup {
    {
        *state.counter.lock().unwrap() += 1;
    }

    html! {
        (state.counter.lock().unwrap())
    }
}

async fn reset(state: State<AppState>) -> Markup {
    {
        *state.counter.lock().unwrap() = 0;
    }

    html! {
        (state.counter.lock().unwrap())
    }
}
