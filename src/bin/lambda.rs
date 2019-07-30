use example_rocket_lamb_api;
use rocket_lamb::{ResponseType, RocketExt};

fn main() {
    example_rocket_lamb_api::rocket()
        .lambda()
        .response_type("image/png", ResponseType::Binary)
        .launch();
}
