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
    pub challenge: String,
}

// validate_attestation
pub async fn validate_attestation(attestation_data: AttestationData) -> &'static str {
    dotenv().ok();
    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let dev: String = env::var("DEV").expect("DEV must be set");
    let prod = if dev == "true" { false } else { true };

    let added = add_challenge(
        attestation_data.challenge.clone(),
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
        &attestation_data.challenge,
        &app_id,
        prod,  // production
        false, // leaf_cert_only
    );

    // If verified
    if verified {
        return "Verified attestation";
    } else {
        return "Invalid attestation";
    }
}
