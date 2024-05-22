use dotenv::dotenv;
use rocket::form::FromForm;
use std::env;

extern crate rocket;

use crate::db::add_challenge;

// #[derive(Deserialize)]
// #[serde(crate = "rocket::serde")]
#[derive(FromForm, Debug)]
pub struct AttestationData {
    pub attestation_string: String,
    pub raw_key_id: String,
    pub challenge: String, // challenge is user-supplied.
}

// validate_attestation
pub async fn validate_attestation(attestation_data: AttestationData) -> &'static str {
    println!("attempting to validate attestation: {:?}", attestation_data);

    dotenv().ok();
    let app_id = env::var("APP_ID").expect("APP_ID must be set");

    // Add challenge to used challenges
    let challenge = String::from_utf8(app_attest::utils::decode_base64_to_bytes(
        &attestation_data.challenge.to_string(),
    ))
    .unwrap();
    let added = add_challenge(
        challenge.clone(),
        attestation_data.attestation_string.clone(),
    )
    .await;

    if added {
        println!("Challenge added.");
    } else {
        println!("Challenge already exists. Invalid attestation.");
        return "challenge already exists";
    }

    // Verify the attestation
    println!("Verifying attestation... {:?}", attestation_data);
    let verified = app_attest::validate_raw_attestation(
        &attestation_data.attestation_string,
        &attestation_data.raw_key_id,
        &challenge,
        &app_id,
        false, // production
        false, // leaf_cert_only
    );

    // If verified
    if verified {
        return "Verified attestation";
    }

    return "";
}
