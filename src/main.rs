#[macro_use]
extern crate rocket;

use std::env;

use rocket::Config;
use std::path::PathBuf;

fn static_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static")
}

#[launch]
fn rocket() -> _ {
    let figment = Config::figment().merge((
        "port",
        env::var("PORT")
            .unwrap_or_else(|_| "8000".into())
            .parse::<u16>()
            .unwrap(),
    ));

    rocket::custom(figment).mount("/", rocket::fs::FileServer::from(static_dir()))
}
