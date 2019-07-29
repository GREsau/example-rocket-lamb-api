use example_rocket_lamb_api;
use rocket_lamb::{lambda, RocketHandler};
use rocket::error::LaunchError;

fn main() -> Result<(), LaunchError> {
    let handler = RocketHandler::new(example_rocket_lamb_api::rocket())?;
    lambda!(handler);
    Ok(())
}
