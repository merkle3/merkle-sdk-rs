use std::env;

use futures::StreamExt;
use merkle_sdk::prelude::Connection;

#[tokio::main]
async fn main() {
    let api_key = env::var("API_KEY").expect("Provide an API KEY");
    let conn = Connection::from_key(api_key).await.expect("connect");
    let mut stream = conn.into_stream();
    while let Some(msg) = stream.next().await {
        println!("{msg:?}");
    }
}
