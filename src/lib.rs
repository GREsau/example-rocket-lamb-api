#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{http::ContentType, response::Content, Rocket};

#[get("/")]
fn hello() -> &'static str {
    "Hello world"
}

// /rocket returns binary data (a PNG image)
#[get("/rocket")]
fn pic() -> Option<Content<&'static [u8]>> {
    let bytes = include_bytes!("../static/rocket.png");
    let content_type = ContentType::new("image", "png");
    Some(Content(content_type, bytes))
}

pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![hello, pic])
}
