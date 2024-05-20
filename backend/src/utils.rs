use rand::Rng;

pub fn generate_random_challenge() -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789";
    let mut rng = rand::thread_rng();

    let output = (0..24)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    output
}

// Function to add a challenge string to the set
pub async fn add_challenge(
    client: &tokio_postgres::Client,
    challenge: &str,
) -> Result<bool, Error> {
    let rows_affected = client
        .execute(
            "INSERT INTO challenges (challenge_string) VALUES ($1) ON CONFLICT DO NOTHING",
            &[&challenge],
        )
        .await?;
    Ok(rows_affected > 0)
}
