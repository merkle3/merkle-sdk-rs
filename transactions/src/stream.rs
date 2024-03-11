use std::{
    pin::Pin,
    task::{Context, Poll},
};

use ethers::types::transaction::eip2718::{TypedTransaction, TypedTransactionError};
use futures::{stream::SplitStream, Stream, StreamExt};
use log::{error, trace};
use rlp::Rlp;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::error::Error;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::{ConnectionBuilder, Transaction};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type StreamItem = Result<Transaction, TxnStreamError>;

const TAG: &str = "transactions::stream";

#[derive(thiserror::Error, Debug)]
pub enum TxnStreamError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Cannot decode transaction: {0}")]
    Decode(#[from] TypedTransactionError),
}

impl From<tokio_tungstenite::tungstenite::Error> for TxnStreamError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        match e {
            Error::Http(response) => {
                let default_msg = "empty response".to_string();
                if let Some(bytes) = response.body() {
                    let msg = String::from_utf8(bytes.to_vec()).unwrap_or(default_msg);
                    Self::Connection(msg)
                } else {
                    Self::Connection(default_msg)
                }
            }
            _ => Self::Connection(e.to_string()),
        }
    }
}

/// Utility struct to acquire a connection to
/// our exposed transaction stream.
///
/// ```rust
///
/// use merkle_sdk_transactions::Connection;
/// use futures::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let api_key = "<API-KEY>";
///     if let Ok(conn) = Connection::with_key(api_key).mainnet().build().await {
///         let mut stream = conn.into_stream();
///         while let Some(msg) = stream.next().await {
///             println!("{msg:?}");
///         }
///     }
/// }
/// ```
pub struct Connection {
    /// This is the raw transactions stream
    /// receiving data from an upstream ws server
    pub(crate) ws_stream: SplitStream<WsStream>,
}

impl Connection {
    pub fn with_key<T: AsRef<str>>(key: T) -> ConnectionBuilder {
        ConnectionBuilder::new(key)
    }
    /// Converts the connection into a stream of transactions
    pub fn into_stream(self) -> Pin<Box<dyn Stream<Item = StreamItem> + Send>> {
        let stream = Transactions::from(self);
        Box::pin(stream)
    }
}

/// This struct implements a stream of pooled transactions.
/// Performs adaptation logics over the bytes stream received
/// from the websocket.
pub struct Transactions {
    /// Inner bytes stream from the websocket
    inner: Pin<Box<dyn Stream<Item = Result<Vec<u8>, Error>> + Send>>,
}

impl From<Connection> for Transactions {
    fn from(conn: Connection) -> Self {
        Transactions {
            inner: conn.ws_stream.map(|msg| msg.map(|m| m.into_data())).boxed(),
        }
    }
}

impl Stream for Transactions {
    type Item = StreamItem;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner.poll_next_unpin(cx) {
            Poll::Ready(msg) => match msg {
                Some(Ok(bytes)) => match TypedTransaction::decode_signed(&Rlp::new(&bytes[..])) {
                    Ok((tx, signature)) => {
                        trace!(target: TAG, "Got txn {tx:?}");
                        Poll::Ready(Some(Ok(Transaction {
                            inner: tx,
                            signature,
                        })))
                    }
                    Err(e) => {
                        let err = Err(TxnStreamError::Decode(e));
                        error!(target: TAG, "Txn decode error: {err:?}");
                        Poll::Ready(Some(err))
                    }
                },
                Some(Err(e)) => {
                    let err = Err(TxnStreamError::from(e));
                    error!(target: TAG, "Connection error: {err:?}");
                    Poll::Ready(Some(err))
                }
                None => {
                    error!(target: TAG, "Txn stream terminated");
                    Poll::Ready(None)
                }
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use tokio_tungstenite::tungstenite::Error;

    use crate::{Connection, Transactions, TxnStreamError};

    #[tokio::test]
    async fn can_handle_connection_error() {
        let wrong_api_key = "foo";
        let conn = Connection::with_key(wrong_api_key).mainnet().build();
        match conn.await {
            Err(crate::TxnStreamError::Connection(_)) => assert!(true),
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn stream_can_decode_txns() {
        let rlp_txn: Vec<u8> = vec![
            2, 248, 107, 1, 4, 132, 1, 241, 4, 152, 133, 4, 197, 251, 94, 222, 130, 82, 8, 148, 42,
            143, 43, 0, 153, 133, 79, 69, 45, 148, 13, 174, 94, 96, 129, 220, 154, 24, 12, 228,
            128, 128, 192, 128, 160, 32, 251, 220, 146, 202, 250, 137, 95, 31, 70, 37, 187, 189,
            168, 76, 151, 225, 168, 234, 183, 111, 97, 191, 191, 177, 35, 110, 20, 240, 49, 144,
            119, 160, 74, 114, 74, 164, 66, 217, 218, 199, 137, 224, 234, 84, 63, 255, 26, 106, 48,
            77, 205, 70, 42, 248, 159, 53, 29, 31, 154, 42, 178, 143, 112, 234,
        ];

        let items: Vec<Result<Vec<u8>, Error>> = vec![Ok(rlp_txn)];
        let stream = futures::stream::iter(items);
        let mut txn_stream = Transactions {
            inner: Box::pin(stream),
        };

        match txn_stream.next().await {
            Some(Ok(_tx)) => assert!(true),
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn stream_can_handle_connection_errors() {
        let items: Vec<Result<Vec<u8>, Error>> = vec![Err(Error::ConnectionClosed)];
        let stream = futures::stream::iter(items);
        let mut txn_stream = Transactions {
            inner: Box::pin(stream),
        };

        match txn_stream.next().await {
            Some(Err(TxnStreamError::Connection(_))) => assert!(true),
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn stream_can_handle_decoding_errors() {
        let wrong_rlp: Vec<u8> = vec![1, 23, 43, 5, 6];
        let items: Vec<Result<Vec<u8>, Error>> = vec![Ok(wrong_rlp)];
        let stream = futures::stream::iter(items);
        let mut txn_stream = Transactions {
            inner: Box::pin(stream),
        };

        match txn_stream.next().await {
            Some(Err(TxnStreamError::Decode(_))) => assert!(true),
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn stream_can_handle_last_message() {
        let items = vec![];
        let stream = futures::stream::iter(items);
        let mut txn_stream = Transactions {
            inner: Box::pin(stream),
        };

        match txn_stream.next().await {
            None => assert!(true),
            _ => unreachable!(),
        }
    }
}
