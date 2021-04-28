use std::fs;
use std::io;
use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom;
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

    for file in files {
        photos.push(file);
    }

    photos.shuffle(&mut thread_rng());

    context.insert("photos", photos);

    Template::render("home/home", &context)
}
