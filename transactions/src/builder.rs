use futures::StreamExt;
use log::debug;
use tokio_tungstenite::connect_async;

use crate::{Connection, TxnStreamError};

const MAINNET: &str = "1";
const POLYGON: &str = "137";
const BASE_URL: &str = "wss://txs.merkle.io/ws";

pub struct ConnectionBuilder {
    key: String,
    chain: String,
    base_url: String,
}

impl ConnectionBuilder {
    pub(crate) fn new<T: AsRef<str>>(key: T) -> Self {
        Self {
            key: key.as_ref().to_string(),
            chain: MAINNET.to_string(),
            base_url: BASE_URL.to_string(),
        }
    }

    pub fn base_url<T: AsRef<str>>(mut self, url: T) -> Self {
        self.base_url = url.as_ref().to_string();
        self
    }

    pub fn polygon(mut self) -> Self {
        self.chain = POLYGON.to_string();
        self
    }

    pub fn mainnet(mut self) -> Self {
        self.chain = MAINNET.to_string();
        self
    }

    pub fn chain(mut self, chain_id: u64) -> Self {
        self.chain = chain_id.to_string();
        self
    }

    pub async fn build(self) ->  Result<Connection, TxnStreamError> {
        let key = self.key;
        let chain = self.chain;
        let base_url = self.base_url;

        let url = format!("{base_url}/ws/{key}/{chain}");

        debug!("Connecting to {url}");
        let (ws_stream, _) = connect_async(url).await?;
        let (_, rlp_stream) = ws_stream.split();
        debug!("Connection established");

        Ok(Connection {
            ws_stream: rlp_stream,
        })
    }
}
