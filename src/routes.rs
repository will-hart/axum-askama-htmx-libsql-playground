use axum::extract::State;
use maud::Markup;

use crate::{components::counter, state::AppState};

pub(crate) async fn home_route_handler(mut state: State<AppState>) -> Markup {
    let current_value = state.get_counter_value().await;
    crate::components::pages::home(current_value)
}

pub(crate) async fn increment_mutation(mut state: State<AppState>) -> Markup {
    counter::counter_value(state.increment_counter().await)
}

pub(crate) async fn reset_mutation(mut state: State<AppState>) -> Markup {
    counter::counter_value(state.reset_counter().await)
}
