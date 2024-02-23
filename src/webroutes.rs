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
                    span #targetme { "0" }

                    button.leftpad hx-post="/increment" hx-target="#targetme" hx-swap="innerHTML" {
                        "Increment"
                    }

                    button.leftpad hx-post="/reset" hx-target="#targetme" hx-swap="innerHTML" {
                        "Reset"
                    }
                }
            }
        }
    )
}
