use dotenv::dotenv;
use rocket::serde::{json::Json, Deserialize};
use std::env;
use tokio_postgres::{Error, NoTls};

use crate::utils::check_and_insert_challenge;

pub mod utils;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/challenge")]
fn challenge() -> String {
    return utils::generate_random_challenge();
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AttestationData<'r> {
    attestation_string: &'r str,
    raw_key_id: &'r str,
    challenge: &'r str, // challenge is user-supplied.
}

#[post("/appattest", format = "application/json", data = "<attestation_data>")]
async fn appattest(attestation_data: Json<AttestationData<'_>>) -> () {
    const APP_ID: &str = "proof-pix";

    dotenv().ok();

    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("connection failed");

    // Spawn a new task to run the connection, so we can execute queries on the client
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Check if challenge has already been used
    match check_and_insert_challenge(&client, attestation_data.challenge).await {
        Ok(_) => {}
        Err(e) => {
            println!("Error checking and inserting challenge: {}", e);
            return;
        }
    };

    let verified = app_attest::validate_raw_attestation(
        attestation_data.attestation_string,
        attestation_data.challenge,
        attestation_data.raw_key_id,
        APP_ID,
        false, // production
        false, // leaf_cert_only
    );

    // TODO: do something with attestation data
    if verified {
        println!("Verified attestation");
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ImageData {
    image_data: Vec<u8>,
    signature: Vec<u8>,
}

#[post("/image", format = "application/json", data = "<image_data>")]
fn post_image(image_data: Json<ImageData>) -> () {
    // TODO: do something with image data
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, challenge, appattest, post_image])
}
