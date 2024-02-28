use std::fs;

pub fn read_media_files(dir: &str) -> std::io::Result<Vec<String>> {
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

pub fn is_image_file(file_name: &str) -> bool {
    file_name.ends_with(".png") || file_name.ends_with(".jpg") || file_name.ends_with(".jpeg") || file_name.ends_with(".gif") || file_name.ends_with(".webp")
}

pub fn is_video_file(file_name: &str) -> bool {
    file_name.ends_with(".mp4") || file_name.ends_with(".webm") || file_name.ends_with(".ogg")
}

pub fn is_audio_file(file_name: &str) -> bool {
    file_name.ends_with(".mp3") || file_name.ends_with(".wav")
}

pub fn is_pdf_file(file_name: &str) -> bool {
    file_name.ends_with(".pdf")
}

pub fn get_video_mime_type(file_name: &str) -> &str {
    if file_name.ends_with(".mp4") {
        "mp4"
    } else if file_name.ends_with(".webm") {
        "webm"
    } else if file_name.ends_with(".ogg") {
        "ogg"
    } else {
        "unknown"
    }
}

pub fn get_audio_mime_type(file_name: &str) -> &str {
    if file_name.ends_with(".mp3") {
        "mpeg"
    } else if file_name.ends_with(".wav") {
        "wav"
    } else {
        "unknown"
    }
}
