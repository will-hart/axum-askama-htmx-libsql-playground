use std::sync::{Arc, Mutex};

use axum::extract::State;
use maud::{html, Markup};

#[derive(Clone, Default)]
pub(crate) struct AppState {
    pub counter: Arc<Mutex<u16>>,
}

pub(crate) async fn increment(state: State<AppState>) -> Markup {
    {
        *state.counter.lock().unwrap() += 1;
    }

    html! {
        (state.counter.lock().unwrap())
    }
}

pub(crate) async fn reset(state: State<AppState>) -> Markup {
    {
        *state.counter.lock().unwrap() = 0;
    }

    html! {
        (state.counter.lock().unwrap())
    }
}
