use std::env;

use futures::StreamExt;
use merkle_sdk::{prelude::Connection, transactions::ChainId};

#[tokio::main]
async fn main() {
    std::env::set_var("API_KEY", "sk_mbs_a35bf8ac729f9992a4319427df6b564f");
    let api_key = env::var("API_KEY").expect("Provide an API KEY");

    // Create a Mainnet connection
    let _conn = Connection::with_key(&api_key)
        .mainnet()
        .build()
        .await
        .expect("connect");

    // Create a Polygon connection
    let _conn = Connection::with_key(&api_key)
        .polygon()
        .build()
        .await
        .expect("connect");

    // Create a BSC connection
    let _conn = Connection::with_key(&api_key)
        .bsc()
        .build()
        .await
        .expect("connect");

    // Create a connection based on chain id
    let conn = Connection::with_key(&api_key)
        .chain(ChainId::Id(137))
        .build()
        .await
        .expect("connect");

    let mut stream = conn.into_stream();
    while let Some(msg) = stream.next().await {
        println!("{msg:?}");
    }
}
