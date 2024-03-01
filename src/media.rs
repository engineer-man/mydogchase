use axum::response::Html;
use std::fs;

pub async fn render_error_page() -> Html<String> {
    match fs::read_to_string("templates/error.html") {
        Ok(contents) => Html(contents),
        Err(_) => Html("<h1>Internal Server Error</h1>".to_string()),
    }
}

fn read_media_files(dir: &str) -> std::io::Result<Vec<String>> {
    let paths = fs::read_dir(dir)?;
    let mut files = Vec::new();
    for path in paths {
        let path = path?.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                files.push(file_name.to_string());
            }
        }
    }
    Ok(files)
}

pub async fn render_html_with_media(file_path: &str, media_dir: &str) -> Html<String> {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            println!("Unable to read file: {}", e);
            return render_error_page().await;
        }
    };
    if let Some(end_body_index) = content.find("\n</body>") {
        let mut media_files = match read_media_files(media_dir) {
            Ok(files) => files,
            Err(_) => {
                println!("Unable to read media directory: {}", media_dir);
                return Html(content);
            }
        };
        alphanumeric_sort::sort_path_slice(&mut media_files);
        let media_insertion_point = content.find("<!-- MEDIA_INSERTION_POINT -->");
        let (indentation, before_media_insertion_point) = if let Some(index) = media_insertion_point {
            let newline_index = content[..index].rfind('\n').unwrap_or(0);
            let indentation = &content[newline_index+1..index];
            (indentation, true)
        } else {
            ("", false)
        };
        let media_tags = media_files.into_iter().enumerate().map(|(i, file)| {
            let indent = if i == 0 || !before_media_insertion_point {
                ""
            } else {
                indentation
            };
            format!("{}<div style=\"background-image: url(/public/chase/{});\"></div>", indent, file)
        }).collect::<Vec<_>>().join("\n");
        let mut new_content = content.clone();
        if let Some(_media_insertion_point) = new_content.find("<!-- MEDIA_INSERTION_POINT -->") {
            new_content = new_content.replacen("<!-- MEDIA_INSERTION_POINT -->", &media_tags, 1);
            Html(new_content)
        } else {
            new_content.insert_str(end_body_index, &media_tags);
            Html(new_content)
        }
    } else {
        Html(content)
    }
}
