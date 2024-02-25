use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/counter/counter.html")]
pub struct Counter {
    pub id: u32,
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/counter/counter_value_update.html")]
pub struct CounterUpdateResponse {
    pub id: u32,
    pub value: u32,
}

impl From<Counter> for CounterUpdateResponse {
    fn from(counter: Counter) -> Self {
        CounterUpdateResponse {
            id: counter.id,
            value: counter.value,
        }
    }
}
