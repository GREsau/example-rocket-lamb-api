#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rust_embed;

use rocket::{http::ContentType, response::Content, Rocket};

#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticFiles;

#[get("/")]
fn hello() -> &'static str {
    "Hello world"
}

// /rocket returns binary data (a PNG image)
#[get("/rocket")]
fn pic() -> Option<Content<Vec<u8>>> {
    let bytes = StaticFiles::get("rocket.png")?.into_owned();
    let content_type = ContentType::new("image", "png");
    Some(Content(content_type, bytes))
}

pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![hello, pic])
}
