use std::env;

use transactions::Connection;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let api_key = env::var("API_KEY").expect("Provide an API KEY");
    let url = format!("wss://txs.merkle.io/ws/{api_key}");
    let conn = Connection::connect(url).await.expect("connect");
    let mut stream = conn.into_stream();
    while let Some(msg) = stream.next().await {
        println!("{msg:?}");
    }
}