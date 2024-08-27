#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from("public"))
        .mount("/", routes![routes::home, routes::favicon])
        .attach(Template::fairing())
}
