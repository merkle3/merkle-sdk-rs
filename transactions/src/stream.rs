use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{stream::SplitStream, Stream, StreamExt};
use log::{error, trace};
use reth_primitives::{TransactionSigned, TransactionSignedEcRecovered};
use reth_rlp::Decodable;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type StreamItem = Result<Option<TransactionSignedEcRecovered>, TxnStreamError>;

const TAG: &str = "transactions::stream";

#[derive(thiserror::Error, Debug)]
pub enum TxnStreamError {
    #[error("Connection error: {0}")]
    Connection(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Cannot decode transaction: {0}")]
    Decode(#[from] reth_rlp::DecodeError),
}

pub struct Connection {
    rlp_stream: SplitStream<WsStream>,
}

impl Connection {
    pub async fn connect<T: AsRef<str>>(url: T) -> Result<Self, TxnStreamError> {
        let url = url.as_ref();
        let (ws_stream, _) = connect_async(url).await?;
        let (_, rlp_stream) = ws_stream.split();
        Ok(Self { rlp_stream })
    }

    pub fn into_stream(self) -> Pin<Box<dyn Stream<Item = StreamItem> + Send>> {
        let stream = TxnsStream::from(self);
        Box::pin(stream)
    }
}

impl From<Connection> for TxnsStream {
    fn from(conn: Connection) -> Self {
        TxnsStream {
            inner: conn.rlp_stream,
        }
    }
}

pub struct TxnsStream {
    inner: SplitStream<WsStream>,
}

impl Stream for TxnsStream {
    type Item = StreamItem;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner.poll_next_unpin(cx) {
            Poll::Ready(msg) => match msg {
                Some(Ok(txn)) => {
                    let bytes: Vec<u8> = txn.into_data();
                    match TransactionSigned::decode(&mut &bytes[..]) {
                        Ok(tx) => {
                            let tx = tx.into_ecrecovered();
                            trace!(target: TAG, "Got txn {tx:?}");
                            Poll::Ready(Some(Ok(tx)))
                        }
                        Err(e) => {
                            let err = Err(TxnStreamError::Decode(e));
                            error!(target: TAG, "Txn decode error: {err:?}");
                            Poll::Ready(Some(err))
                        }
                    }
                }
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
