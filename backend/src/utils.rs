use rand::Rng;
pub async fn check_and_insert_challenge(
    client: &tokio_postgres::Client,
    challenge: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the challenge already exists
    let row = client
        .query_opt(
            "SELECT 1 FROM challenges WHERE challenge_string = $1",
            &[&challenge],
        )
        .await?;

    if row.is_some() {
        return Err(Box::from("Challenge already exists."));
    } else {
        // Insert the new challenge
        client
            .execute(
                "INSERT INTO challenges (challenge_string) VALUES ($1)",
                &[&challenge],
            )
            .await?;
        println!("Challenge added.");
    }

    Ok(())
}

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
