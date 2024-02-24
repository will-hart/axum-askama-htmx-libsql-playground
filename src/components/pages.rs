use super::{
    common::head,
    counter::{counter_value, increment_button},
};
use maud::{html, Markup, DOCTYPE};

pub(crate) fn home(current_value: u16) -> Markup {
    html!(
        (DOCTYPE)
        html {
            (head("Counter"))

            body {
                .intro {
                    span { "Counter " }
                    span.counter-button #targetme { (counter_value(current_value)) }
                    span.htmx-indicator #spinner { " Saving" }

                    (increment_button("Increment", "/increment"))
                    (increment_button("Reset", "/reset"))
                }
            }
        }
    )
}
