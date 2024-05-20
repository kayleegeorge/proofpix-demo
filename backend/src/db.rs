use dotenv::dotenv;
use redis::Commands;
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use std::env;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client};
use sha2::{Digest, Sha256};

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ImageRequest {
    pub photo_bytes: Vec<u8>,
    pub timestamp: String,
    pub photo_signature: String,
    pub poster_pubkey: String,
    pub poster_attest_proof: String,
    pub location: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Image {
    pub photo_url: String,
    pub timestamp: String,
    pub photo_signature: String,
    pub poster_pubkey: String,
    pub poster_attest_proof: String,
    pub location: String,
}

// Connect to Redis
pub async fn connect_to_redis() -> redis::RedisResult<redis::Connection> {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let client = redis::Client::open(redis_url).expect("Failed to connect to Redis");
    let connection = client.get_connection().unwrap();
    Ok(connection)
}

// Get all images
pub async fn get_all_images() -> Vec<Image> {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    let mut images: Vec<Image> = Vec::new();
    // get all images
    images
}

// Post an image to DB
pub async fn post_image(image_data: Image) -> bool {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    // Key with timestamp
    let key = format!("image:{}", image_data.timestamp.clone());
    let json_data = serde_json::to_string(&image_data).expect("Failed to serialize data");

    let _: () = con
        .set(key, json_data)
        .expect("Failed to set data in Redis");

    true
}

// Connect S3 client
async fn get_s3_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

// Upload an image to S3
pub async fn upload_image(image_data: ImageRequest) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();

    let bucket = env::var("S3_BUCKET").expect("S3_BUCKET must be set");
    let s3_client = get_s3_client().await;

    // Hash the contents as filename
    let image_data_str = serde_json::to_string(&image_data).expect("Failed to serialize data");
    let mut hasher = Sha256::new();
    hasher.update(image_data_str);
    let hash = hasher.finalize();
    let file_name = format!("{:x}", hash);

    let byte_stream = ByteStream::from(image_data.photo_bytes.clone());

    // Upload the image to S3
    s3_client
        .put_object()
        .bucket(bucket)
        .key(&file_name)
        .body(byte_stream)
        .send()
        .await?;

    Ok(file_name)
}
