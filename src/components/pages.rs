use askama::Template;

use super::counter::Counter;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    title: String,
    counters: Vec<Counter>,
}

pub(crate) fn index(counters: Vec<Counter>) -> IndexTemplate {
    IndexTemplate {
        title: "Counter".to_owned(),
        counters,
    }
}
