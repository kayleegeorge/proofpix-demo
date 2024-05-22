use db::{Image, ImageMetadata, ImagesResponse};
use dotenv::dotenv;
use rocket::{form::Form, serde::json::Json};
use std::env;

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
fn challenge() -> Json<String> {
    return Json(utils::generate_random_challenge());
}

// Upload an image to S3 and add its metadata to cache
#[post("/add", data = "<image_data_form>")]
async fn post_image(image_data_form: Form<ImageRequest<'_>>) -> Json<String> {
    let image_data = image_data_form.into_inner();
    let file_name = db::upload_image(image_data.photo_file).await.unwrap();

    dotenv().ok();
    let bucket = env::var("S3_BUCKET").expect("S3_BUCKET must be set");

    let photo_url = format!("https://{bucket}.s3.us-west-2.amazonaws.com/{file_name}");

    // Add the image to the cache
    let image_metadata = ImageMetadata {
        photo_url: photo_url,
        timestamp: image_data.timestamp,
        photo_signature: image_data.photo_signature,
        poster_pubkey: image_data.poster_pubkey,
        poster_attest_proof: image_data.poster_attest_proof,
        location_lat: image_data.location_lat,
        location_long: image_data.location_long,
    };

    db::add_image(file_name.clone(), image_metadata).await;
    Json(file_name)
}

// Get all images
#[get("/images")]
async fn fetch_all_images() -> Json<ImagesResponse> {
    let images = db::get_all_images().await;
    let post_data_objects: ImagesResponse = ImagesResponse {
        post_data_objects: images,
    };
    Json(post_data_objects)
}

// Get all image URLs
#[get("/filenames")]
async fn fetch_all_filenames() -> Json<Vec<String>> {
    let urls = db::get_all_filenames().await;
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
async fn app_attest(data: Form<AttestationData>) -> Json<String> {
    println!("Received attestation form: {:?}", data);
    let attestation_data = data.into_inner();
    println!("Parsed attestation data.");

    return Json(
        appattest::validate_attestation(attestation_data)
            .await
            .to_string(),
    );
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
            fetch_all_filenames,
            app_attest
        ],
    )
}
