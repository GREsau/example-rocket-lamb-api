use example_rocket_lamb_api;
use rocket::error::LaunchError;
use rocket_lamb::{lambda, ResponseType, RocketHandler};

fn main() -> Result<(), LaunchError> {
    let handler = RocketHandler::new(example_rocket_lamb_api::rocket())?
        .response_type("image/png", ResponseType::Binary);
    lambda!(handler);
    Ok(())
}
