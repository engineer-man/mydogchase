use std::fs;
use axum::{routing::{get, get_service}, http::StatusCode, response::{Html, IntoResponse}, Router};
use tower_http::services::ServeDir;
use crate::media::{render_html_with_media};

async fn not_found() -> impl IntoResponse {
    let file_path = "templates/error.html";
    let custom_404_html = fs::read_to_string(file_path).unwrap_or_else(|_| {
    String::from(r#"
<!doctype html>
<html lang="en-US">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=yes" />
    <title>guacamole</title>
    <link rel="stylesheet" type="text/css" href="https://thomasf.github.io/solarized-css/solarized-dark.min.css"></link>
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body>
    <h1>ERROR</h1>
    <p>You shouldn't be here. Please go away.</p>
</body>
</html>
"#)
    });
    (StatusCode::NOT_FOUND, Html(custom_404_html))
}

fn routes_static() -> Router {
    Router::new().nest_service("/static", get_service(ServeDir::new("static")))
}

pub fn app() -> Router {
    let mut router = Router::new()
        .merge(routes_static())
        .route("/favicon.ico", get_service(ServeDir::new("./static")))
        .fallback(get(not_found));
    router = router.route("/", get(move || {
        let file = "templates/home/home.html";
        let media = "public/chase";
        async move {
            render_html_with_media(&file, &media).await
        }
    }));
    router = router
        .nest_service(&format!("/public/chase"), ServeDir::new("public/chase/"));
    router
}
