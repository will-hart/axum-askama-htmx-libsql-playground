use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "partials/counter.html")]
pub struct Counter {
    pub id: u32,
    pub value: u32,
}

pub(crate) fn counter_value(counter: &Counter) -> String {
    format!("{}", counter.value).to_owned()
}
