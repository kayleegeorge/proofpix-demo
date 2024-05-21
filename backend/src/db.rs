use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client};
use dotenv::dotenv;
use redis::Commands;
use rocket::fs::TempFile;
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::io;
use sha2::{Digest, Sha256};
use std::env;

// Connect to Redis
pub async fn connect_to_redis() -> redis::RedisResult<redis::Connection> {
    dotenv().ok();
    let dev = env::var("DEV").expect("DEV must be set");
    let mut redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    if dev == "true" {
        redis_url = env::var("DEV_REDIS_URL").expect("DEV_REDIS_URL must be set");
    }

    let client = redis::Client::open(redis_url).expect("Failed to connect to Redis");
    let connection = client
        .get_connection()
        .expect("Failed to get Redis connection");

    Ok(connection)
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ImageMetadata {
    pub timestamp: String,
    pub photo_signature: String,
    pub poster_pubkey: String,
    pub poster_attest_proof: String,
    pub location_lat: String,
    pub location_long: String,
}

// Get image data for one image given a url
pub async fn get_image_metadata(file_name: String) -> Option<ImageMetadata> {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    dotenv().ok();
    let bucket = env::var("S3_BUCKET").expect("S3_BUCKET must be set");

    // image url formatted as "s3://bucket/file_name"
    let image_url = format!("s3://{}/{}", bucket, file_name);

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

#[derive(FromForm, Debug)]
pub struct ImageRequest<'r> {
    pub photo_file: TempFile<'r>,
    pub timestamp: String,
    pub photo_signature: String,
    pub poster_pubkey: String,
    pub poster_attest_proof: String,
    pub location_lat: String,
    pub location_long: String,
}

// Add an image's metadata to cache
pub async fn add_image(file_name: String, image_metadata: ImageMetadata) -> bool {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    let data_string = serde_json::to_string(&image_metadata).expect("Failed to serialize data");

    let _: () = con
        .set(file_name, data_string)
        .expect("Failed to set data in Redis");

    true
}

// Connect S3 client
async fn get_s3_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not set");
    let aws_secret_access_key =
        env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not set");

    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

// Upload an image to S3
pub async fn upload_image(photo_file: TempFile<'_>) -> Result<String, Box<dyn std::error::Error>> {
    println!("Connecting to S3");
    dotenv().ok();
    let bucket = env::var("S3_BUCKET").expect("S3_BUCKET must be set");
    let s3_client = get_s3_client().await;

    println!("Serializing image data");

    // Read photo bytes
    let file = photo_file;
    let mut buffer = Vec::new();
    let mut stream = file.open().await?;
    io::copy(&mut stream, &mut buffer).await?;

    let byte_stream = ByteStream::from(buffer.clone());
    println!("Byte stream from image: {:?}", byte_stream);

    // photo name is hash of the photo signature for now
    let mut hasher = Sha256::new();
    hasher.update(buffer.clone());
    let hash = hasher.finalize();
    let file_name = format!("{:x}", hash);

    // Upload the image to S3
    println!("Uploading image to S3");
    s3_client
        .put_object()
        .bucket(bucket)
        .key(&file_name)
        .body(byte_stream)
        .send()
        .await?;

    println!("Done: Image uploaded to S3!");
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
    pub location_lat: String,
    pub location_long: String,
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
            location_lat: metadata.location_lat,
            location_long: metadata.location_long,
        };
        images.push(image);
    }

    images
}

// Check if challenge / attestation exists. If not, add to the cache
// TODO: change to proof instead of attestation_string
pub async fn add_challenge(challenge: String, attestation_string: String) -> bool {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    // Check if challenge exists
    let challenge_proof: bool = con
        .exists(challenge.clone())
        .expect("Failed to check if challenge exists");

    if challenge_proof {
        return false;
    } else {
        let _: () = con
            .set(challenge, attestation_string)
            .expect("Failed to set data in Redis");
        return true;
    }
}
