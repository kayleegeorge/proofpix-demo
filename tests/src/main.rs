use image::io::Reader as ImageReader;
use image::ImageFormat;
use reqwest::Client;
use serde_json::json;
use std::fs::File;
use std::io::{BufReader, Cursor};
use tokio;

#[macro_use]
extern crate rocket;

async fn post_image(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    const URL: &str = "https://appattest-demo.onrender.com/add";

    // Handle image serialization
    println!("Processing image...");
    let img_path = "./bogota.jpg";
    let img_file = File::open(img_path)?;
    let reader = BufReader::new(img_file);
    let img = ImageReader::new(reader).with_guessed_format()?.decode()?;

    println!("Serializing image...");
    let mut img_bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut img_bytes);
    img.write_to(&mut cursor, ImageFormat::Jpeg)?;

    println!("Making request...");
    // dummy data
    let body = json!({
        "photo_bytes": img_bytes,
        "timestamp": "October 2023",
        "photo_signature": "value3",
        "poster_pubkey": "value4",
        "poster_attest_proof": "value5",
        "location": "Bogota, Colombia",
    });

    let response = client
        .post(URL)
        .json(&body) // Set the request body
        .send() // Send the request
        .await?; // Await the response

    println!("Response: {:?}", response.status());
    if response.status().is_success() {
        println!("Success! Response: {:?}", response.text().await?);
    } else {
        println!("Error: {:?}", response.status());
    }

    Ok(())
}

async fn _get_image(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    const URL: &str = "https://appattest-demo.onrender.com/image/9fc4285b5f554b2c3c9f5bfdd14bbe216416cf5f9ef55706ba5cfb67f18b6424"; // TODO add file name

    // Make the GET request
    let response = client
        .get(URL)
        .send() // Send the request
        .await?; // Await the response

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Success! Response: {}", body);
    } else {
        println!("Error: {:?}", response.status());
    }

    Ok(())
}

async fn get_urls(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    const URL: &str = "https://appattest-demo.onrender.com/urls";

    // Make the GET request
    let response = client
        .get(URL)
        .send() // Send the request
        .await?; // Await the response

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Success! Response: {}", body);
    } else {
        println!("Error: {:?}", response.status());
    }

    Ok(())
}

// #[tokio::test]
async fn test_functions() {
    let client = Client::new();

    // Test post image
    let res = post_image(&client).await.expect("Failed to post image");
    println!("1: {:?}", res);

    // Test get all image urls
    let res = get_urls(&client).await.expect("Failed to get image urls");
    println!("2: {:?}", res);
}

#[get("/test")]
async fn test() -> &'static str {
    test_functions().await;
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 9797)))
        .mount("/", routes![test])
}
