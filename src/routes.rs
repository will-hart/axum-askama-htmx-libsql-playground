use axum::extract::State;
use maud::Markup;

use crate::{components::counter, state::AppState};

pub(crate) async fn home_route_handler() -> Markup {
    crate::components::pages::home()
}

pub(crate) async fn increment_mutation(mut state: State<AppState>) -> Markup {
    state.increment_counter();
    counter::counter_value(*state.counter.lock().unwrap())
}

pub(crate) async fn reset_mutation(mut state: State<AppState>) -> Markup {
    state.reset_counter();
    counter::counter_value(*state.counter.lock().unwrap())
}
