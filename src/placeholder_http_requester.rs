use reqwest::{Client, Error};
use serde_json::json;

pub async fn fetch_placeholder_json() -> Result<(), Error> {
    let client = Client::new();
    let url = "https://jsonplaceholder.typicode.com/posts";

    let payload = json!({
        "title": "Learn Rust!",
        "body": "Less talk let's Rust!",
        "userId": 1071
    });

    let response = client.post(url)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Failed to send data. Status: {}", response.status());
    }

    Ok(())
}