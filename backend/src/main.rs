use db::Image;
use rocket::serde::json::serde_json;
use rocket::serde::{self, json::Json, Deserialize, Serialize};
use rocket_contrib::{Value, JSON};
use rocket_sync_db_pools::database;

use crate::utils::add_challenge;

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

// Appattest endpoint
#[post("/appattest", format = "application/json", data = "<attestation_data>")]
async fn appattest(attestation_data: Json<AttestationData<'_>>) -> () {
    const APP_ID: &str = "proof-pix";

    let con = utils::get_db_client().await;

    // Add challenge to used challenges
    let added = add_challenge(&con, attestation_data.challenge)
        .await
        .expect("Failed to add challenge");

    if added {
        println!("Challenge added.");
    } else {
        println!("Challenge already exists.");
        return;
    }

    let verified = app_attest::validate_raw_attestation(
        attestation_data.attestation_string,
        attestation_data.challenge,
        attestation_data.raw_key_id,
        APP_ID,
        false, // production
        false, // leaf_cert_only
    );

    // If verified
    if verified {
        println!("Verified attestation");
        // can do something
    }
}

// Post request for an image
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ImageRequest {
    timestamp: String,
    photo_url: String,
    photo_signature: String,
    poster_pubkey: String,
    poster_attest_proof: String,
    location: String,
}

// Add an image
#[post("/add", format = "application/json", data = "<image_data>")]
async fn post_image(image_data: Json<ImageRequest>) -> () {
    let con = utils::get_db_client().await;
    db::add_image(&con, image_data).await;
}

// Get all images
#[get("/images")]
async fn get_all_images() -> JSON<Vec<Image>> {
    let con = db::get_db_client().await;
    let images = db::get_images(&con).await;
    Json(images)
}

// Get all images from a specific user
#[get("/images/<pub_key>")]
async fn get_images_for_user(pub_key: String) -> JSON<Vec<Image>> {
    let con = db::get_db_client().await;
    let images = db::get_images_for_user(&con, user_id).await;
    Json(images)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            challenge,
            appattest,
            post_image,
            get_images_for_user,
            get_all_images
        ],
    )
}
