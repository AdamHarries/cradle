use ::libcradle::protocol;
// use axum::http::response;

use reqwest;
// use reqwest::IntoUrl;
// use serde::de::DeserializeOwned;
// use serde::Serialize;
// use std::collections::HashMap;

async fn play_song<T: ToString>(
    client: &reqwest::Client,
    song: T,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = protocol::Enqueue {
        filepath: song.to_string(),
    };
    let resp = client
        .post("http://127.0.0.1:3000/enqueue")
        .json(&data)
        .send()
        .await?
        .json::<protocol::EnqueueResponse>()
        .await?;
    println!("Response: {:#?}", resp);
    Ok(())
}

// async fn send_command<U: IntoUrl, T: Serialize + ?Sized, R: DeserializeOwned>(
//     client: &reqwest::Client,
//     url: U,
//     data: &T,
// ) -> Result<R, reqwest::Error> {
//     client.post(url).json(data).send().await?.json::<R>().await
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Client started");
    let client = reqwest::Client::new();
    play_song(&client, "Jumpin' at the woodside").await?;
    play_song(&client, "Lester leaps in").await?;
    play_song(&client, "Take The A train").await?;
    play_song(&client, "Johnny Come Lately").await?;
    play_song(&client, "Tain't What You Do It").await?;

    Ok(())
}
