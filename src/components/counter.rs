use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub id: u32,
    pub value: u32,
}

pub(crate) fn counter_value(counter: &Counter) -> String {
    format!("{}", counter.value).to_owned()
}
