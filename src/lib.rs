#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello world"
}

pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![hello])
}
