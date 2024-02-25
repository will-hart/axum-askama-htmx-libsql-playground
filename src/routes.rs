use axum::extract::State;

use crate::{
    components::{self, counter},
    state::AppState,
};

pub(crate) async fn home_route_handler(
    mut state: State<AppState>,
) -> components::pages::IndexTemplate {
    let current_value = state.get_counter_value().await;
    crate::components::pages::index(current_value)
}

pub(crate) async fn increment_mutation(mut state: State<AppState>) -> String {
    counter::counter_value(state.increment_counter().await)
}

pub(crate) async fn reset_mutation(mut state: State<AppState>) -> String {
    counter::counter_value(state.reset_counter().await)
}
