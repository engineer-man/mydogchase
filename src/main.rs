#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::fs;
use std::io;
use std::io::{stdout, Write};
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[get("/")]
fn home() -> Template {
    let mut context = HashMap::new();

    let mut files = fs::read_dir("public/chase")
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

    context.insert("photos", photos);

    Template::render("home/home", &context)
}

fn main() {
    rocket::ignite()
        .mount("/public", StaticFiles::from("public"))
        .mount("/", routes![home])
        .attach(Template::fairing())
        .launch();
}
