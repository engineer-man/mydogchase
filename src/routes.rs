use std::fs;
use std::io;
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::response::NamedFile;

#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").ok()
}

#[get("/")]
pub fn home() -> Template {
    let mut context = HashMap::new();

    let files = fs::read_dir("public/chase")
        .unwrap()
        .map(|result| {
            result.map(|file| {
                file.path()
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let mut photos = vec![];
    println!("files {:?}", files);

    for file in files {
        photos.push(file);
    }
    alphanumeric_sort::sort_path_slice(&mut photos);
    println!("photos {:?}", photos);

    context.insert("photos", photos);

    Template::render("home/home", &context)
}
