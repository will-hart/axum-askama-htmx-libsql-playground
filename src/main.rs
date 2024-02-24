use axum::{
    routing::{get, post},
    Router,
};

mod components;
mod routes;
mod state;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = Router::new()
        .route("/", get(routes::home_route_handler))
        .route("/increment", post(routes::increment_mutation))
        .route("/reset", post(routes::reset_mutation))
        .with_state(state::AppState::default());

    Ok(app.into())
}
