use super::counter::{counter_value, increment_button};
use maud::{html, Markup, PreEscaped, DOCTYPE};

pub(crate) fn home() -> Markup {
    html!(
        (DOCTYPE)
        html {
            head {
                (PreEscaped("<script src=\"https://unpkg.com/htmx.org@1.9.10\" crossorigin=\"anonymous\"></script>"))
                (PreEscaped("<style>.leftpad { margin-left: 1em; }</style>"))
                title { "Fred" }
            }
            body {
                .intro {
                    span { "Counter " }
                    span.counter-button #targetme { (counter_value(0)) }
                    span.htmx-indicator #spinner { " Saving" }

                    (increment_button("Increment", "/increment"))
                    (increment_button("Reset", "/reset"))
                }
            }
        }
    )
}
