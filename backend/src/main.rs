use std::borrow::Borrow;

use db::{Image, ImageMetadata};
use rocket::Data;

use rocket::{form::Form, http::ContentType, serde::json::Json};

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
#[post("/add", data = "<image_data_form>")]
async fn post_image(image_data_form: Form<ImageRequest<'_>>) -> Json<String> {
    println!("Received image form: {:?}", image_data_form);
    let image_data = image_data_form.into_inner();
    let file_name = db::upload_image(image_data.photo_file).await.unwrap();

    // Add the image to the cache
    let image_metadata = ImageMetadata {
        timestamp: image_data.timestamp,
        photo_signature: image_data.photo_signature,
        poster_pubkey: image_data.poster_pubkey,
        poster_attest_proof: image_data.poster_attest_proof,
        location: image_data.location,
    };

    db::add_image(file_name.clone(), image_metadata).await;
    Json(file_name)
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
#[post("/appattest", data = "<data>")]
async fn app_attest(data: Form<AttestationData>) -> String {
    println!("Received ttestation form: {:?}", data);
    let attestation_data = data.into_inner();
    return appattest::validate_attestation(attestation_data)
        .await
        .to_string();
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
