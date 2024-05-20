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

// Get a random challenge
#[get("/challenge")]
fn challenge() -> String {
    return utils::generate_random_challenge();
}

// Post an images
#[post("/add", format = "application/json", data = "<image_data>")]
async fn post_image(image_data: Json<ImageRequest>) -> () {
    let file_name = db::upload_image(image_data).await;

    // Image to add to Redis
    let image = Image {
        photo_url: file_name,
        timestamp: image_data.timestamp,
        photo_signature: image_data.photo_signature,
        poster_pubkey: image_data.poster_pubkey,
        poster_attest_proof: image_data.poster_attest_proof,
        location: image_data.location,
    };
    db::post_image(image_data).await;
}

// Get all images
#[get("/images")]
async fn fetch_all_images() -> Json<Vec<Image>> {
    let images = db::get_all_images().await;
    Json(images)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, challenge, post_image, fetch_all_images])
}
