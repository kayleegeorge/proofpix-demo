use rocket::serde::{json::Json, Deserialize};

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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AttestationData<'r> {
    attestation_string: &'r str,
    raw_key_id: &'r str,
}

#[post("/appattest", format = "application/json", data = "<attestation_data>")]
fn appattest(attestation_data: Json<AttestationData<'_>>) -> () {
    const APP_ID: &str = "proof-pix";
    const challenge: &str = "get-challenge"; // TODO: get challenge from db.

    let verified = app_attest::validate_raw_attestation(
        attestation_data.attestation_string,
        challenge,
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
