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
pub struct UpdateCounterResponse {
    pub id: u32,
    pub value: u32,
}

impl From<Counter> for UpdateCounterResponse {
    fn from(counter: Counter) -> Self {
        UpdateCounterResponse {
            id: counter.id,
            value: counter.value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/counter/add_counter.html")]
pub struct AddCounterResponse {
    pub id: u32,
    pub value: u32,
}

impl From<Counter> for AddCounterResponse {
    fn from(counter: Counter) -> Self {
        AddCounterResponse {
            id: counter.id,
            value: counter.value,
        }
    }
}
