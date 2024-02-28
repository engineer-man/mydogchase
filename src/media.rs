use axum::response::Html;
use std::fs;
use crate::utils::{
    read_media_files,
    is_image_file, is_video_file, is_audio_file, is_pdf_file,
    get_video_mime_type, get_audio_mime_type
};
use crate::solarized::{print_fancy, RED, BOLD, PrintMode::NewLine};

pub async fn render_error_page() -> Html<String> {
    match fs::read_to_string("templates/error.html") {
        Ok(contents) => Html(contents),
        Err(_) => Html("<h1>Internal Server Error</h1>".to_string()),
    }
}

pub async fn render_html_with_media(file_path: &str, media_dir: &str, media_route: &str) -> Html<String> {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            print_fancy(&[
                ("Error: Unable to read file: ", RED, vec![]),
                (file_path, RED, vec![BOLD]),
                (&format!(" - {}", e), RED, vec![])
            ], NewLine);
            return render_error_page().await;
        }
    };
    if let Some(end_body_index) = content.find("\n</body>") {
        let mut media_files = match read_media_files(media_dir) {
            Ok(files) => files,
            Err(_) => {
                print_fancy(&[
                    ("Error: Unable to read media directory: ", RED, vec![]),
                    (media_dir, RED, vec![BOLD]),
                ], NewLine);
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
            if is_video_file(&file) {
                format!("{}<video controls><source src='/public/{}/{}' type='video/{}'></video>{}", indent, media_route, file, get_video_mime_type(&file), file)
            } else if is_audio_file(&file) {
                format!("{}<audio controls><source src='/public/{}/{}' type='audio/{}'></audio>", indent, media_route, file, get_audio_mime_type(&file))
            } else if is_pdf_file(&file) {
                format!("{}<iframe src='/public/{}/{}' width='100%' height='600px'></iframe>", indent, media_route, file)
            } else if is_image_file(&file) {
                format!("{}<div style=\"background-image: url(/public/{}/{});\"></div>", indent, media_route, file)
            } else {
                format!("")
            }
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

pub async fn render_html(file_path: &str) -> Html<String> {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            print_fancy(&[
                ("Error: Unable to read file: ", RED, vec![]),
                (file_path, RED, vec![BOLD]),
                (&format!(" - {}", e), RED, vec![])
            ], NewLine);
            return render_error_page().await;
        }
    };
    Html(content)
}
