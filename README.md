<img src="public/merkle-large.png" width="80" height="80" style="border-radius: 4px"/>

# merkle Rust SDK

The merkle SDK is a great way to access our products.

## Install

Add the following to your cargo.toml file:

```toml
[dependencies]
merkle-sdk = "0.0.7"
```

## Examples

Examples are organized into individual crates under the `/examples` folder.
You can run any of the examples by executing:

```bash
# cargo run -p <example-crate-name> --example <name>
cargo run -p examples-transactions --example transactions
```

## Listen to transactions

Get an API key for free at [mbs.merkle.io](https://mbs.merkle.io).

```rust
use merkle_sdk::prelude::Connection;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let api_key = "sk_mbs_a35bf8ac729f9992a4319427df6b564f";
    if let Ok(conn) = Connection::from_key(api_key).await {
        let mut stream = conn.into_stream();
        while let Some(txn) = stream.next().await {
            println!("{txn:?}");
        }
     }
}
```

This library supports connections to different chains. This can be achieved using out of box builder functions:

```rust
use std::env;

use futures::StreamExt;
use merkle_sdk::{prelude::Connection, transactions::ChainId};

#[tokio::main]
async fn main() {
    let api_key = "sk_mbs_a35bf8ac729f9992a4319427df6b564f"

    // Create a Mainnet connection
    let _conn_res = Connection::with_key(&api_key)
        .mainnet()
        .build()
        .await;

    // Create a Polygon connection
    let _conn_res = Connection::with_key(&api_key)
        .polygon()
        .build()
        .await;

    // Create a BSC connection
    let _conn_res = Connection::with_key(&api_key)
        .bsc()
        .build()
        .await;

    // Create a connection based on chain id
    let conn_res = Connection::with_key(&api_key)
        .chain(ChainId::Id(137))
        .build()
        .await;

    if let Ok(conn) = conn_res {
        let mut stream = conn.into_stream();
        while let Some(txn) = stream.next().await {
            println!("{txn:?}");
        }
     }
}

```