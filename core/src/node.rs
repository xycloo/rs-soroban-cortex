use bytes::Bytes;
use std::sync::{Arc, Mutex};

pub enum Events {
    LockedInBridge(LockedInBridge),
}

type Bytes32 = [u8; 32];

pub struct LockedInBridge {
    hash: Bytes32,
    recipient: Bytes32,
    amount: i128,
}

impl LockedInBridge {
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
}

pub struct Node {
    bridge_in_contract: Bytes32,
    bridge_out_contract: Bytes32,

    // events in the latest
    in_events_queue: Arc<Mutex<Vec<LockedInBridge>>>,

    // bytes representation of the function name used to
    // load information of the bridge event in the out contract.
    bridge_out_contract_txload_function: Bytes,
}

// Define the trait
pub trait EventProcessor {
    fn bridge_in_contract_events_logger(&self);
    fn process_event(&self, event: LockedInBridge);
    fn process_event_queue(&self);
}

// Implement the trait for Node
impl EventProcessor for Node {
    fn bridge_in_contract_events_logger(&self) {
        // TODO: Implement logger from an Ethereum contract
        // Reference implementation help: https://github.com/tomusdrw/rust-web3/blob/master/examples/contract_log_filter.rs

        // TODO: Add read logs to logs queue as `Event::LockedInBridge(parsed_log)`
    }

    fn process_event(&self, event: LockedInBridge) {
        // TODO: Implement event processing logic
    }

    fn process_event_queue(&self) {
        loop {
            let mut queue = self.in_events_queue.lock().unwrap();

            if let Some(event) = queue.pop() {
                self.process_event(event);
            }
        }
    }
}

impl Node {
    pub fn set_oracle_data(&self) {}
}
