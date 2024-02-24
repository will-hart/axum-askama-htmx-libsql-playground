use maud::Markup;

pub(crate) async fn home_route_handler() -> Markup {
    crate::components::pages::home()
}
