use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub(crate) struct AppState {
    pub counter: Arc<Mutex<u16>>,
}

impl AppState {
    pub(crate) fn increment_counter(&mut self) {
        *self.counter.lock().unwrap() += 1;
    }

    pub(crate) fn reset_counter(&mut self) {
        *self.counter.lock().unwrap() = 0;
    }
}
