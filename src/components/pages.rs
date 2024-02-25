use askama::Template;

use super::counter::Counter;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    title: String,
    id: u32,
    current_value: u32,
}

pub(crate) fn index(counter: Counter) -> IndexTemplate {
    IndexTemplate {
        title: "Counter".to_owned(),
        id: counter.id,
        current_value: counter.value,
    }
}
