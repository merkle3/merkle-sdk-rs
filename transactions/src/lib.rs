mod builder;
mod stream;
pub use builder::{ChainId, ChainName, ConnectionBuilder};
use ethers::types::{transaction::eip2718::TypedTransaction, Bytes, Signature};
pub use stream::{Connection, Transactions, TxnStreamError};

/// Wrapper struct for a `ethers-rs` transaction plus signature.
#[derive(Debug)]
pub struct Transaction {
    /// The inner transaction.
    pub inner: TypedTransaction,
    /// Inner transaction signature.
    pub signature: Signature,
}

impl Transaction {
    /// Returns the RLP bytes representation of the transaction.
    /// These [`Bytes`] represent a signed transaction that can be
    /// relayed to the network.
    pub fn rlp_signed(&self) -> Bytes {
        self.inner.rlp_signed(&self.signature)
    }
}
