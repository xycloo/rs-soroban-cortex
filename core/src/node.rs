use bytes::Bytes;
use log::{info, debug};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use async_trait::async_trait;
use std::rc::Rc;
use tokio::task::yield_now;

use crate::rpc::NodeStellarRpcClient;

pub enum Events {
    LockedInBridge(LockedInBridge),
}

pub type Bytes32 = [u8; 32];

#[derive(Debug)]
pub struct LockedInBridge {
    hash: Bytes32,
    recipient: Bytes32,
    amount: i128,
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
}

pub struct Node<'a> {
    // contract of the chain initiating the swap.
    in_contract: Bytes32,
    
    // events in the latest
    in_events_queue: Arc<Mutex<Vec<LockedInBridge>>>,

    // wrapper around the rpc client to interact with Soroban
    stellar_rpc: NodeStellarRpcClient<'a>
}

// Define the trait
#[async_trait]
pub trait EventProcessor {
    async fn bridge_in_contract_events_logger(&self);
    async fn process_event(&self, event: LockedInBridge);
    async fn process_event_queue(&self);
}

// Implement the trait for Node
#[async_trait]
impl<'a> EventProcessor for Node<'a> {
    async fn bridge_in_contract_events_logger(&self) {
        // TODO: Implement logger from an Ethereum contract
        // Reference implementation help: https://github.com/tomusdrw/rust-web3/blob/master/examples/contract_log_filter.rs

        // TODO: Add read logs to logs queue as `Event::LockedInBridge(parsed_log)`

        // Test implementation
        let address = stellar_strkey::ed25519::PublicKey::from_string("GAICVK2SYRLD7YFKD3D2TZGKAB6NH34VP4NW2ZYEEQHLLZOQIVC5VXEL").unwrap();
        let event = LockedInBridge::new([219, 29, 40, 188, 191, 213, 181, 129, 93, 39, 8, 89, 7, 230, 165, 232, 72, 141, 15, 63, 124, 182, 125, 121, 2, 208, 54, 237, 51, 247, 70, 207], address.0, 1000);
        self.in_events_queue.lock().unwrap().push(event);

        sleep(Duration::from_secs(2)).await;
        
        // drop lock
    }


    async fn process_event(&self, event: LockedInBridge) {
        // TODO: Implement event processing logic

        debug!("processing new event");

        let payload = event.serialize_to_bytes();
        
        // broadcast payload
        let tx = self.stellar_rpc.build_tx(payload).await;
        
        self.stellar_rpc.send_transaction(tx).await
    }

    async fn process_event_queue(&self) {
        let popped;        
        {
            // get lock
            let mut queue = self.in_events_queue.lock().unwrap();
            popped = queue.pop();
            // drop lock
        }
        
        if let Some(event) = popped {
            self.process_event(event).await;
        }
    }    
}

impl<'a> Node<'a> {
    /// Sets the initial parameters of the node and configurates the object.    
    pub fn new(node_secret: &'a str, in_contract: Bytes32, out_contract: Bytes32, stellar_rpc_endpoint: &'a str, txload_function: &'a str, network_passphrase: &'a str) -> Self {
        let client = NodeStellarRpcClient::new(
            node_secret, 
            network_passphrase, 
            stellar_rpc_endpoint, 
            out_contract, 
            txload_function
        );
        
        Self { in_contract, in_events_queue: Default::default(), stellar_rpc: client }
    }

    pub async fn run(&self) {
        info!("[+] starting service");

        loop {
            self.process_event_queue().await;
            self.bridge_in_contract_events_logger().await;
        }
    }

    pub fn set_oracle_data(&self) {}
}
