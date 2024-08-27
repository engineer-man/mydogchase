use std::fs;
use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rocket_dyn_templates::{Template, context};
use rocket::fs::NamedFile;

#[get("/favicon.ico")]
pub async fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").await.ok()
}

#[get("/")]
pub fn home() -> Template {

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

    Template::render("home/home", context! {
        photos: photos,
    })
}
