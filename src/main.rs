#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer, NamedFile};

use std::env;

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", env::var("PORT").unwrap_or_else(|_| "8083".into()).parse::<u16>().unwrap()));

    rocket::custom(figment)
        .mount("/", FileServer::from(relative!("static")).rank(0))
}