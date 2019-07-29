#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rust_embed;

use rocket::{http::ContentType, response::Content, Rocket};

#[get("/")]
fn hello() -> &'static str {
    "Hello world"
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticFiles;

#[get("/pic")]
fn pic() -> Option<Content<Vec<u8>>> {
    let bytes = StaticFiles::get("rocket.png")?.into_owned();
    Some(Content(ContentType::new("image", "png"), bytes))
}

pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![hello, pic])
}
