use std::sync::{Arc, Mutex};

use axum::{
    routing::{delete, get, post},
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

    conn.execute(include_str!("../migrations/1_setup_tables.sql"), ())
        .await
        .unwrap();
    conn.execute(
        include_str!("../migrations/2_insert_initial_counter.sql"),
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
        .route("/counter", post(routes::add_counter_mutation))
        .route("/counter/:id", delete(routes::delete_counter_mutation))
        .route("/increment/:id", post(routes::increment_mutation))
        .route("/reset/:id", post(routes::reset_mutation))
        .with_state(state);

    Ok(app.into())
}
