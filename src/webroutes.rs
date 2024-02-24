use maud::{html, Markup, PreEscaped, DOCTYPE};

pub(crate) async fn home() -> Markup {
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
                    span.counter-button #targetme { "0" }
                    span.htmx-indicator #spinner { " Saving" }

                    button.leftpad.counter-button
                        hx-post="/increment"
                        hx-indicator="#spinner"
                        hx-disabled-elt=".counter-button"
                        hx-target="#targetme"
                        hx-swap="innerHTML"
                    {
                        "Increment"
                    }

                    button.leftpad.counter-button
                        hx-post="/reset"
                        hx-indicator="#spinner"
                        hx-disabled-elt=".counter-button"
                        hx-target="#targetme"
                        hx-swap="innerHTML"
                    {
                        "Reset"
                    }


                }
            }
        }
    )
}
