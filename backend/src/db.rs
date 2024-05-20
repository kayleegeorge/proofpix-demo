use dotenv::dotenv;
use postgres::{Connection, TlsMode};
use rocket_contrib::{Value, JSON};
use std::env;
use tokio_postgres::{Error, NoTls};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Image {
    id: iu32,
    timestamp: String,
    photo_url: String,
    photo_signature: String,
    poster_pubkey: String,
    poster_attest_proof: String,
    location: String,
}

// Get a PostgreSQL client
pub async fn get_db_client() -> tokio_postgres::Client {
    dotenv().ok();

    // Get the database URL from the environment variable
    let database_url = env::var("DB_URL").expect("DB_URL must be set");

    // Connect to the PostgreSQL database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("DB connection failed");

    // Spawn a new task to manage the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

// Get all images.
pub fn get_all_images() -> Vec<Image> {
    let con = get_db_client().unwrap();
    let rows = &con.query("SELECT * FROM images").unwrap();
    let mut images: Vec<Image> = Vec::new();

    for row in rows {
        let id: i32 = row.get("id");
        let timestamp: String = row.get("timestamp");
        let photo_url: String = row.get("photo_url");
        let photo_signature: String = row.get("photo_signature");
        let poster_pubkey: String = row.get("poster_pubkey");
        let poster_attest_proof: String = row.get("poster_attest_proof");
        let location: String = row.get("location");

        images.push(Image {
            id,
            timestamp,
            photo_url,
            photo_signature,
            poster_pubkey,
            poster_attest_proof,
            location,
        });
    }

    images
}

// Add an image to the database.
pub async fn add_image(client: &tokio_postgres::Client, image_data: ImageRequest) {
    let rows_affected = client
        .execute(
            "INSERT INTO images (timestamp, photo_url, photo_signature, poster_pubkey, poster_attest_proof, location) VALUES ($1, $2, $3, $4, $5, $6)",
            &[&image_data.timestamp, &image_data.photo_url, &image_data.photo_signature, &image_data.poster_pubkey, &image_data.poster_attest_proof, &image_data.location],
        )
        .await?;
    Ok(rows_affected > 0)
}

// Get all images for a specific user
pub async fn get_images_for_user(client: &tokio_postgres::Client, pub_key: String) -> Vec<Image> {
    let con = get_db_client().unwrap();
    let rows = &con
        .query("SELECT * FROM images WHERE poster_pubkey = $1", &[&pub_key])
        .unwrap();
    let mut images: Vec<Image> = Vec::new();

    for row in rows {
        let id: i32 = row.get("id");
        let timestamp: String = row.get("timestamp");
        let photo_url: String = row.get("photo_url");
        let photo_signature: String = row.get("photo_signature");
        let poster_pubkey: String = row.get("poster_pubkey");
        let poster_attest_proof: String = row.get("poster_attest_proof");
        let location: String = row.get("location");

        images.push(Image {
            id,
            timestamp,
            photo_url,
            photo_signature,
            poster_pubkey,
            poster_attest_proof,
            location,
        });
    }

    images
}
