use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    components::{
        self,
        counter::{self, Counter},
    },
    state::AppState,
};

pub(crate) async fn home_route_handler(
    mut state: State<AppState>,
) -> components::pages::IndexTemplate {
    let counters = state.get_counters().await;
    crate::components::pages::index(counters)
}

pub(crate) async fn add_counter_mutation(mut state: State<AppState>) -> Counter {
    state.create_counter().await
}

pub(crate) async fn delete_counter_mutation(
    Path(id): Path<u32>,
    mut state: State<AppState>,
) -> StatusCode {
    if id == 1 {
        return StatusCode::FORBIDDEN;
    }

    match state.delete_counter(id).await {
        Ok(_) => StatusCode::OK,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub(crate) async fn increment_mutation(Path(id): Path<u32>, mut state: State<AppState>) -> String {
    counter::counter_value(&state.increment_counter(id).await)
}

pub(crate) async fn reset_mutation(Path(id): Path<u32>, mut state: State<AppState>) -> String {
    counter::counter_value(&state.reset_counter(id).await)
}
