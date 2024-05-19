pub mod utils;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/challenge")]
fn challenge() -> String {
    // TODO: store teh challenge in the db.
    return utils::generate_random_challenge();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, challenge])
}
