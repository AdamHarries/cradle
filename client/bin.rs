use::libcradle::protocol;

use reqwest;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = protocol::Enqueue {
        filepath: "jumpin' at the woodside".to_string(),
    };
    println!("Client started");
    let client = reqwest::Client::new();
    let resp = client.post("http://127.0.0.1:3000/enqueue")
        .json(&data)
        .send()
        .await?
        .json::<protocol::EnqueueResponse>()
        .await?;
    println!("Response: {:#?}", resp);
    Ok(())
}