use db::Image;
use rocket::serde::json::{self, serde_json};
use rocket::serde::{self, json::Json, Deserialize, Serialize};

pub mod db;
pub mod utils;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AttestationData<'r> {
    attestation_string: &'r str,
    raw_key_id: &'r str,
    challenge: &'r str, // challenge is user-supplied.
}

// Get a random challenge
#[get("/challenge")]
fn challenge() -> String {
    return utils::generate_random_challenge();
}

// Add an image
#[post("/add", format = "application/json", data = "<image_data>")]
async fn post_image(image_data: Json<Image>) -> () {
    db::post_image(image_data).await;
}

// Get all images.
#[get("/images")]
async fn get_all_images() -> Json<Vec<Image>> {
    let images = db::get_all_images().await;
    Json(images)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, challenge, post_image, get_all_images])
}
