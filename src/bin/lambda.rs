use example_rocket_lamb_api;
use rocket_lamb::RocketExt;

fn main() {
    example_rocket_lamb_api::rocket().lambda().launch();
}
