use db::{Image, ImageMetadata};
use rocket::serde::json::Json;

use crate::{appattest::AttestationData, db::ImageRequest};

pub mod appattest;
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

// Upload an image to S3 and add its metadata to cache
#[post("/add", format = "application/json", data = "<image_data_json>")]
async fn post_image(image_data_json: Json<ImageRequest>) -> () {
    let image_data = image_data_json.into_inner();
    let file_name = db::upload_image(image_data.clone()).await.unwrap();
    db::add_image(file_name, image_data).await;
}

// Get all images
#[get("/images")]
async fn fetch_all_images() -> Json<Vec<Image>> {
    let images = db::get_all_images().await;
    Json(images)
}

// Get all image URLs
#[get("/urls")]
async fn fetch_all_urls() -> Json<Vec<String>> {
    let urls = db::get_all_urls().await;
    Json(urls)
}

// Get an image metadata via its URL
#[get("/image/<file_name>")]
async fn get_image(file_name: String) -> Json<Option<ImageMetadata>> {
    let image = db::get_image_metadata(file_name).await;
    match image {
        Some(image) => Json(Some(image)),
        None => Json(None),
    }
}

// Appattest endpoint
#[post("/appattest", format = "application/json", data = "<attestation_data>")]
async fn app_attest(attestation_data: Json<AttestationData>) -> () {
    let attestation_data = attestation_data.into_inner();
    appattest::validate_attestation(attestation_data).await;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            challenge,
            post_image,
            fetch_all_images,
            get_image,
            fetch_all_urls,
            app_attest
        ],
    )
}
