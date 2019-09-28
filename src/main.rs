#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[get("/")]
fn home() -> Template {
    let mut context = HashMap::new();

    let photos = vec![
        "IMG_20180912_095622.jpg",
        "IMG_20180912_212514.jpg",
        "IMG_20180914_124612.jpg",
        "IMG_20180917_095616.jpg",
        "IMG_20180918_101453.jpg",
        "IMG_20180920_231331.jpg",
        "IMG_20180928_121138.jpg",
        "IMG_20180929_140615.jpg",
        "IMG_20180930_123317.jpg",
        "IMG_20181002_131520.jpg",
        "IMG_20181006_225617.jpg",
        "IMG_20181007_174948.jpg",
        "IMG_20181011_163032.jpg",
        "IMG_20181012_235245.jpg",
        "IMG_20181018_211610.jpg",
        "IMG_20181019_105839.jpg",
        "IMG_20181021_165252.jpg",
        "IMG_20181025_102242.jpg",
        "IMG_20181031_194229.jpg",
        "IMG_20181102_114038.jpg",
        "IMG_20181105_102651.jpg",
        "IMG_20181105_183137.jpg",
        "IMG_20181106_233230.jpg",
        "IMG_20181107_225418.jpg"
    ];

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
