#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod routes;

fn main() {
    rocket::ignite()
        .mount("/public", StaticFiles::from("public"))
        .mount("/", routes![routes::home])
        .attach(Template::fairing())
        .launch();
}
