const TMPL: &str = include_str!("template.html");

pub(crate) fn endpoint() -> impl poem::Endpoint + 'static {
    poem::endpoint::make_sync(|_| poem::web::Html(TMPL))
}