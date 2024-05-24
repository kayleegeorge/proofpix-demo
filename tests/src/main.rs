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
    // const URL: &str = "http://127.0.0.1:8000/appattest";

    // Create a form with the attestation data
    let form = multipart::Form::new()
        .text("attestation_string", "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")
        .text("raw_key_id", "z3ze3nY0WShtvsflzslqAmhSwI4+3JO8kAJRgYv3tt0=")
        .text("challenge", "\"RnSjUYwSJtLIPVCDBr3JOA2r\"");

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
    // post_image(&client).await.expect("Failed to post image");

    // // Test get all image urls
    // let res = get_urls(&client).await.expect("Failed to get image urls");
    // println!("2: {:?}", res);

    test_appattest(&client)
        .await
        .expect("Failed to test appattest");
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
