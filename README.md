<img src="public/merkle-large.png" width="80" height="80" style="border-radius: 4px"/>

# merkle Rust SDK

The merkle SDK is a great way to access our products.

## Install

Add the following to your cargo.toml file:

```toml
[dependencies]
merkle-sdk = { git = "git@github.com:merkle3/merkle-sdk-rs.git" }
```

## Examples

Examples are organized into individual crates under the `/examples` folder.
You can run any of the examples by executing:

```bash
# cargo run -p <example-crate-name> --example <name>
cargo run -p examples-transactions --example transactions
```

## Listen to transactions

```rust
use merkle_sdk_transactions::Connection;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let api_key = "<SOME_API_KEY>";
    if let Ok(conn) = Connection::from_key(api_key).await {
        let mut stream = conn.into_stream();
        while let Some(txn) = stream.next().await {
            println!("{txn:?}");
        }
     }
}
```