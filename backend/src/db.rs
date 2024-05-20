use dotenv::dotenv;
use redis::Commands;
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use std::env;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client};
use sha2::{Digest, Sha256};

// Connect to Redis
pub async fn connect_to_redis() -> redis::RedisResult<redis::Connection> {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let client = redis::Client::open(redis_url).expect("Failed to connect to Redis");
    let connection = client.get_connection().unwrap();
    Ok(connection)
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ImageMetadata {
    pub timestamp: String,
    pub photo_signature: String,
    pub poster_pubkey: String,
    pub poster_attest_proof: String,
    pub location: String,
}

// Get image data for one image given a url
pub async fn get_image_metadata(image_url: String) -> Option<ImageMetadata> {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    let data_string: String = match con.get(image_url) {
        Ok(json) => json,
        Err(_) => return None,
    };

    // Deserialize the JSON string to an ImageMetadata struct
    match serde_json::from_str(&data_string) {
        Ok(image_metadata) => Some(image_metadata),
        Err(_) => None,
    }
}

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

// Add an image's metadata to cache
pub async fn add_image(image_url: String, image_data: ImageRequest) -> bool {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    // Convert the image data to string
    let image_data_json = ImageMetadata {
        timestamp: image_data.timestamp,
        photo_signature: image_data.photo_signature,
        poster_pubkey: image_data.poster_pubkey,
        poster_attest_proof: image_data.poster_attest_proof,
        location: image_data.location,
    };
    let data_string = serde_json::to_string(&image_data_json).expect("Failed to serialize data");

    let _: () = con
        .set(image_url, data_string)
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

// Return all image URLs in the cache
pub async fn get_all_urls() -> Vec<String> {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    let urls: Vec<String> = con.keys("*").expect("Failed to get keys from Redis");

    urls
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

// Get all data from cache
pub async fn get_all_images() -> Vec<Image> {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    let keys: Vec<String> = con.keys("*").expect("Failed to get keys from Redis");

    // Fetch the image metadata for each image
    let mut images: Vec<Image> = Vec::new();
    for key in keys {
        let value: String = con.get(&key).expect("Failed to get value from Redis");
        let metadata: ImageMetadata =
            serde_json::from_str(&value).expect("Failed to deserialize JSON");
        let image = Image {
            photo_url: key,
            timestamp: metadata.timestamp,
            photo_signature: metadata.photo_signature,
            poster_pubkey: metadata.poster_pubkey,
            poster_attest_proof: metadata.poster_attest_proof,
            location: metadata.location,
        };
        images.push(image);
    }

    images
}
