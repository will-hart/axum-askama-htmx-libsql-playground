use maud::{html, Markup};

pub(crate) fn increment_button(title: &str, url: &str) -> Markup {
    html! {
        button.leftpad.counter-button
            hx-post=(url)
            hx-indicator="#spinner"
            hx-disabled-elt=".counter-button"
            hx-target="#targetme"
            hx-swap="innerHTML"
        {
            (title)
        }
    }
}

pub(crate) fn counter_value(value: u16) -> Markup {
    html! {
        (value)
    }
}
