//! Types that deal with receiving and broadcasting messages within chains.

use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;

// Events enum, should not be used outside of the crate.
pub(crate) enum Events {
    LockedInBridge(LockedInBridge),
}

/// Alias for soroban contract ids.
pub type Bytes32 = [u8; 32];

/// Message event that the node needs to broadcast to Soroban.
/// Might not be needed if we remove serialization and deserialization and just pass the message.
/// Serialization and deserialization might be removed since:
/// - currently it's only used to check the integrity of the message.
/// - the node should trust the implementor to stream the events of the required topic
///   and the implementor sohuld trust the contract to emit valid events
/// - even if the implementor doesn't trust the contract, they have a resources limit (maximum fee).
#[derive(Debug)]
pub struct LockedInBridge {
    hash: Bytes32,
    recipient: Bytes32,
    amount: i128,
}

fn bytes_to_i128(bytes: [u8; 16]) -> i128 {
    let mut result: i128 = 0;

    for i in (0..16).rev() {
        result <<= 8;
        result |= bytes[i] as i128;
    }

    // Check if the original value was negative
    if bytes[0] & 0x80 != 0 {
        result = !result + 1;
        result *= -1;
    }

    result
}

impl LockedInBridge {
    pub(crate) fn new(hash: Bytes32, recipient: Bytes32, amount: i128) -> Self {
        Self { hash, recipient, amount }
    }
    
    pub(crate) fn serialize_to_bytes(&self) -> [u8; 80] {
        let mut bytes = [0u8; 80];
        bytes[..32].copy_from_slice(&self.hash);
        bytes[32..64].copy_from_slice(&self.recipient);

        let value = self.amount;
        let mut abs_value = if value < 0 { !value + 1 } else { value };

        for i in 64..80 {
            bytes[i] = (abs_value & 0xFF) as u8;
            abs_value >>= 8;
        }

        bytes
    }

    pub(crate) fn deserialize_from_bytes(bytes: [u8; 80]) -> Self {
        let mut hash: [u8; 32] = [0; 32];
        let mut address: [u8; 32] = [0; 32];
        let mut amount: [u8; 16] = [0; 16];

        hash.copy_from_slice(&bytes[0..32]);
        address.copy_from_slice(&bytes[32..64]);
        amount.copy_from_slice(&bytes[64..80]);

        Self { 
            hash, 
            recipient: address,
            amount: bytes_to_i128(amount)
        }
    }
}

/// Node errors.
pub enum NodeError {
    /// Node is unable to deserialize the message
    Conversion
}

/// Trait that every [`I`] must implement in order to be read and broadcast to Soroban by the node.
pub trait TryIntoMessage {
    /// Tries to convert the implementor's type to 80 bytes.
    fn try_into(self) -> Result<[u8; 80], NodeError>;
}

/// Trait that the node's listener must implement in order for the node to process the events stream from a contract.
#[async_trait]
pub trait EventLogger<I>: Sync + Send { // TODO: probably rename
    /// TODO: possibly remove this method
    fn new(contract_address: &[u8]) -> Self // TODO: probably remove params as the implementor
                                            // will always need to put their own logic.
        where Self: Sized;
    
    /// Reads the events stream. Implementor must return a stream that polls the server every [`poll_interval`].
    /// The interval is currently set by the node, but will probably change to being selected by the implementor.
    /// 
    /// TODO: remove poll_interval arg on trait
    async fn read_stream(&self, poll_interval: std::time::Duration) -> Pin<Box<(dyn Stream<Item = I> + std::marker::Send + 'static)>>;
}
