use reqwest;
use serde_json::Value;

async fn get_gmail_profile(user_id: &str, access_token: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://gmail.googleapis.com/gmail/v1/users/{}/profile", user_id);

    let response = client.get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let profile: Value = response.json().await?;
    Ok(profile)
}

// Example usage in your main function or another async function
async fn example_usage() {
    let user_id = "me"; // Use "me" for the authenticated user
    let access_token = "ya29.a0AcM612xFPVYXtkwIH6LT4wICsA1qxGtAqXrFkgS4Qdeu3CScbXJzdjwicRR-K_dYPlyQbktXHHLxyQWKfrPrNc_KFxHcuTVFIUQfkrR_IqA20XYHQe9pPPIfL3esLnPcHtdlWNp0cah7MZ00WY4pnwZuFaX02Jvxf0mbR9-VaCgYKAVkSARESFQHGX2MiygS4JkDXFsiFursHuSKOPQ0175";

    match get_gmail_profile(user_id, access_token).await {
        Ok(profile) => println!("Gmail profile: {:?}", profile),
        Err(e) => eprintln!("Error fetching Gmail profile: {}", e),
    }
}

#[tokio::main]
async fn main() {
    example_usage().await;
}
