use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use libsql::Builder;
use shuttle_secrets::SecretStore;

mod components;
mod routes;
mod state;

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let url = if let Some(secret) = secret_store.get("TURSO_ADDR") {
        secret.to_owned()
    } else {
        panic!("Can't find DB_ADDR secret")
    };

    let token = if let Some(secret) = secret_store.get("TURSO_TOKEN") {
        secret.to_owned()
    } else {
        panic!("Can't find DB_TOKEN secret")
    };

    let db = Builder::new_remote(url, token).build().await.unwrap();
    let conn = db.connect().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS counter (
        id INT PRIMARY KEY AUTOINCREMENT,
        count INT NOT NULL DEFAULT 0
    )",
        (),
    )
    .await
    .unwrap();

    let mut state = state::AppState {
        db: Arc::new(Mutex::new(db)),
    };
    println!("COUNT: {:?}", state.get_counter_value().await);

    let app = Router::new()
        .route("/", get(routes::home_route_handler))
        .route("/increment", post(routes::increment_mutation))
        .route("/reset", post(routes::reset_mutation))
        .with_state(state);

    Ok(app.into())
}
