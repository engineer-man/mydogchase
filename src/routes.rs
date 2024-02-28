use std::fs;
use axum::{routing::{get, get_service}, http::StatusCode, response::{Html, IntoResponse}, Router};
use tower_http::services::ServeDir;
use crate::config::Config;
use crate::media::{render_html, render_html_with_media};

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

pub fn app(config: &Config) -> Router {
    let mut router = Router::new()
        .merge(routes_static())
        .fallback(get(not_found));
    for (path, settings) in &config.routes {
        if let Some(file_path) = settings.get(0) {
            let media_route = path.trim_start_matches('/');
            if let Some(media_dir) = settings.get(1) {
                let file_clone = file_path.clone();
                let media_dir_clone = media_dir.clone();
                let media_route_clone = media_route.to_string();
                router = router.route(path, get(move || {
                    let file = file_clone.clone();
                    let media = media_dir_clone.clone();
                    let route = media_route_clone.clone();
                    async move {
                        render_html_with_media(&file, &media, &route).await
                    }
                }));
                let serve_dir = ServeDir::new(media_dir);
                router = router
                    .nest_service(&format!("/public/{}", media_route), serve_dir)
            } else {
                let file_clone = file_path.clone();
                router = router.route(path, get(move || {
                    async move {
                        render_html(&file_clone).await
                    }
                }));
            }
        }
    }
    router
}
