// Appattest endpoint
// #[post("/appattest", format = "application/json", data = "<attestation_data>")]
// async fn appattest(attestation_data: Json<AttestationData<'_>>) -> () {
//     const APP_ID: &str = "proof-pix";

//     // Add challenge to used challenges
//     let added = add_challenge(&con, attestation_data.challenge)
//         .await
//         .expect("Failed to add challenge");

//     if added {
//         println!("Challenge added.");
//     } else {
//         println!("Challenge already exists.");
//         return;
//     }

//     let verified = app_attest::validate_raw_attestation(
//         attestation_data.attestation_string,
//         attestation_data.challenge,
//         attestation_data.raw_key_id,
//         APP_ID,
//         false, // production
//         false, // leaf_cert_only
//     );

//     // If verified
//     if verified {
//         println!("Verified attestation");
//         // can do something
//     }
// }