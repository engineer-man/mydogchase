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
