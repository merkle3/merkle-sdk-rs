use futures::StreamExt;
use log::debug;
use tokio_tungstenite::connect_async;

use crate::{Connection, TxnStreamError};

const BASE_URL: &str = "wss://txs.merkle.io/ws";

pub enum ChainId {
    Named(ChainName),
    Id(u64),
}

pub enum ChainName {
    Mainnet,
    Polygon,
    Bsc,
}

impl ChainId {
    pub fn as_u64(&self) -> u64 {
        match self {
            ChainId::Named(chain) => chain.as_u64(),
            ChainId::Id(id) => *id,
        }
    }
}

impl ChainName {
    pub fn as_u64(&self) -> u64 {
        match self {
            ChainName::Mainnet => 1,
            ChainName::Polygon => 137,
            ChainName::Bsc => 56,
        }
    }
}

/// Builder struct for [`Connection`] instances.
pub struct ConnectionBuilder {
    key: String,
    chain: u64,
    base_url: String,
}

impl ConnectionBuilder {
    pub(crate) fn new<T: AsRef<str>>(key: T) -> Self {
        Self {
            key: key.as_ref().to_string(),
            chain: ChainName::Mainnet.as_u64(),
            base_url: BASE_URL.to_string(),
        }
    }

    /// Sets the connection's base url. This is generally not required
    /// but we leave this setting open to increase flexibility.
    /// Example:
    ///
    /// ```rust
    ///
    /// use merkle_sdk_transactions::Connection;
    /// use futures::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api_key = "<API-KEY>";
    ///     if let Ok(conn) = Connection::with_key(api_key).base_url("wss://txs.merkle.io/ws").build().await {
    ///         let mut stream = conn.into_stream();
    ///         while let Some(msg) = stream.next().await {
    ///             println!("{msg:?}");
    ///         }
    ///     }
    /// }
    /// ```
    pub fn base_url<T: AsRef<str>>(mut self, url: T) -> Self {
        self.base_url = url.as_ref().to_string();
        self
    }

    pub fn mainnet(mut self) -> Self {
        self.chain = ChainName::Mainnet.as_u64();
        self
    }

    pub fn polygon(mut self) -> Self {
        self.chain = ChainName::Polygon.as_u64();
        self
    }

    pub fn bsc(mut self) -> Self {
        self.chain = ChainName::Bsc.as_u64();
        self
    }

    pub fn chain(mut self, chain_id: ChainId) -> Self {
        self.chain = chain_id.as_u64();
        self
    }

    pub async fn build(self) -> Result<Connection, TxnStreamError> {
        let key = self.key;
        let chain = self.chain;
        let base_url = self.base_url;

        let url = format!("{base_url}/{key}/{chain}");

        debug!("Connecting to {url}");

        let (ws_stream, _) = connect_async(url).await.map_err(TxnStreamError::from)?;
        let (_, rlp_stream) = ws_stream.split();
        debug!("Connection established");

        Ok(Connection {
            ws_stream: rlp_stream,
        })
    }
}
