use axum::response::Html;

pub mod oauth;
pub mod superuser;

pub(crate) async fn index() -> Html<String> {
    // Read the content of the HTML file at compile time
    let html_content = include_str!("./index.html");

    // Return the HTML content wrapped in an Html response
    Html(html_content.to_string())
}
