use axum::extract::{Path, State};

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

pub(crate) async fn increment_mutation(Path(id): Path<u32>, mut state: State<AppState>) -> String {
    counter::counter_value(&state.increment_counter(id).await)
}

pub(crate) async fn reset_mutation(Path(id): Path<u32>, mut state: State<AppState>) -> String {
    counter::counter_value(&state.reset_counter(id).await)
}
