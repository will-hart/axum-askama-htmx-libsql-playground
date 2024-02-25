use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    title: String,
    current_value: u16,
}

pub(crate) fn index(current_value: u16) -> IndexTemplate {
    IndexTemplate {
        title: "Counter".to_owned(),
        current_value,
    }
}
