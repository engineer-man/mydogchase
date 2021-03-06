#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate alphanumeric_sort;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod routes;

fn main() {
    rocket::ignite()
        .mount("/public", StaticFiles::from("public"))
        .mount("/", routes![routes::home, routes::favicon])
        .attach(Template::fairing())
        .launch();
}
