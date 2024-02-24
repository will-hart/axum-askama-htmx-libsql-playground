use maud::{html, Markup, PreEscaped};

pub(crate) fn head(title: &str) -> Markup {
    html!(
        head {
            (PreEscaped("<script src=\"https://unpkg.com/htmx.org@1.9.10\" crossorigin=\"anonymous\"></script>"))
            (PreEscaped("<style>.leftpad { margin-left: 1em; }</style>"))
            title { "HTMX Test Site | " (title) }
        }
    )
}
