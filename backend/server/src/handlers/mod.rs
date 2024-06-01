use axum::response::Html;
pub mod oauth;
pub mod superuser;

pub(crate) async fn index() -> Html<String> {
    let html_content = include_str!("./index.html");

    Html(html_content.to_string())
}
