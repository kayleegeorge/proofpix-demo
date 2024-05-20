use dotenv::dotenv;
use redis::AsyncCommands;
use rocket::serde::json::serde_json;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Image {
    timestamp: String,
    photo_url: String,
    photo_signature: String,
    poster_pubkey: String,
    poster_attest_proof: String,
    location: String,
}

// Connect to Redis
pub(crate) async fn connect_to_redis() -> redis::RedisResult<redis::aio::Connection> {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let client = redis::Client::open(redis_url)?;
    let connection = client.get_connection().unwrap();
    Ok(connection)
}

// Get all images
pub async fn get_all_images() -> Vec<Image> {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    let mut images: Vec<Image> = Vec::new();

    let mut iter: redis::AsyncIter<'_, String> =
        con.scan_match("image:*").expect("Failed to scan keys");

    while let Some(key) = iter.next_item().await {
        let value: String = con.get(&key).await.expect("Failed to get value from Redis");
        let image: Image = serde_json::from_str(&value).expect("Failed to deserialize JSON");
        images.push(image);
    }

    images
}

// Post an image
pub async fn post_image(image_data: Json<Image>) -> &'static str {
    let mut con = connect_to_redis()
        .await
        .expect("Failed to connect to Redis");

    // Key with pub key
    let key = format!("image:{}", image_data.poster_pubkey.clone());
    let json_data =
        serde_json::to_string(&image_data.into_inner()).expect("Failed to serialize data");

    let _: () = con
        .set(key, json_data)
        .expect("Failed to set data in Redis");

    "Image added successfully"
}
