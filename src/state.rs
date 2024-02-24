use std::sync::{Arc, Mutex};

use axum::extract::State;
use maud::Markup;

use crate::components::counter;

#[derive(Clone, Default)]
pub(crate) struct AppState {
    pub counter: Arc<Mutex<u16>>,
}

pub(crate) async fn increment(state: State<AppState>) -> Markup {
    {
        *state.counter.lock().unwrap() += 1;
    }

    counter::counter_value(*state.counter.lock().unwrap())
}

pub(crate) async fn reset(state: State<AppState>) -> Markup {
    {
        *state.counter.lock().unwrap() = 0;
    }

    counter::counter_value(*state.counter.lock().unwrap())
}
