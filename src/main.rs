use axum::{
    routing::{get, post},
    Router,
};

mod state;
mod webroutes;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = Router::new()
        .route("/", get(webroutes::home))
        .route("/increment", post(state::increment))
        .route("/reset", post(state::reset))
        .with_state(state::AppState::default());

    Ok(app.into())
}
