use reqwest::multipart;
use reqwest::Client;
use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate rocket;

// example POST request with file and text fields
async fn post_image(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    // const URL: &str = "https://appattest-demo.onrender.com/add";
    const URL: &str = "http://127.0.0.1:8000/add";

    // Handle image serialization
    println!("Processing image...");
    let img_path = "./bogota.jpg";
    let mut img_file = File::open(img_path)?;
    let mut buffer: Vec<u8> = Vec::new();
    img_file.read_to_end(&mut buffer)?;

    println!("Making request...");
    let form = multipart::Form::new()
        .part(
            "photo_file",
            multipart::Part::bytes(buffer)
                .file_name("bogota.jpg")
                .mime_str("image/jpeg")?,
        )
        .text("timestamp", "value1")
        .text("photo_signature", "value2")
        .text("poster_pubkey", "value3")
        .text("poster_attest_proof", "value4")
        .text("location", "Bogota, Colombia");

    let response = client
        .post(URL)
        .multipart(form)
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

async fn test_appattest(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    const URL: &str = "https://appattest-demo.onrender.com/appattest";

    // Create a form with the attestation data
    let form = multipart::Form::new()
        .text("attestation_string", "value1")
        .text("raw_key_id", "value2")
        .text("challenge", "value3");

    // Send the request
    let response = client
        .post(URL)
        .multipart(form)
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

    // // Test post image
    post_image(&client).await.expect("Failed to post image");

    // // Test get all image urls
    // let res = get_urls(&client).await.expect("Failed to get image urls");
    // println!("2: {:?}", res);

    // test_appattest(&client)
    //     .await
    //     .expect("Failed to test appattest");
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
