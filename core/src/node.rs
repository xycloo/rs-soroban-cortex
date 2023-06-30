use futures::{FutureExt, Stream, StreamExt, stream::Next};
use log::{info, debug};
use web3::types::Filter;
use std::{sync::{Arc, Mutex}, pin::Pin};
use tokio::time::{sleep, Duration};
use async_trait::async_trait;

use crate::rpc::NodeStellarRpcClient;

pub enum Events {
    LockedInBridge(LockedInBridge),
}

pub type Bytes32 = [u8; 32];
pub type H160 = [u8; 20];

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


#[async_trait]
pub trait EventLogger<I>: Sync + Send {
    fn new(contract_address: &[u8]) -> Self
        where Self: Sized;
    async fn read_stream(&self, poll_interval: std::time::Duration) -> Pin<Box<(dyn Stream<Item = I> + std::marker::Send + 'static)>>;
}

pub struct LogRaw {
    pub data: Vec<u8>
}

pub mod test_listener_eth {
    use std::pin::Pin;

    use async_trait::async_trait;
    use web3::types::Filter;
    use super::{EventLogger, LogRaw};
    use web3::futures::StreamExt;

    #[derive(Clone)]
    pub struct MyListener {
        filter: Filter
    }

    #[async_trait]
    impl EventLogger<std::result::Result<web3::types::Log, web3::Error>> for MyListener
     {
        fn new(contract_address: &[u8]) -> Self {
            let eth_filter = web3::types::FilterBuilder::default()
            .address(vec![web3::types::H160(contract_address.try_into().unwrap())])
            .topics(
                Some(vec![hex_literal::hex!(
                    "d282f389399565f3671145f5916e51652b60eee8e5c759293a2f5771b8ddfd2e"
                )
                .into()]),
                None,
                None,
                None,
            )
            .build();

            Self { filter: eth_filter }
        }

        async fn read_stream(&self, poll_interval: std::time::Duration) -> Pin<Box<(dyn futures::Stream<Item = std::result::Result<web3::types::Log, web3::Error>> + Send + 'static)>> {
            let web3 = web3::Web3::new(web3::transports::Http::new("http://localhost:8545").unwrap());
            let filter = web3.eth_filter().create_logs_filter(self.filter.clone()).await;
            filter.unwrap().stream(poll_interval).boxed()
        }
    }

    // TODO: function that parses `data` to `bytes` to be deserialized into the ['LockedInBridge'] object.
    impl From<web3::types::Log> for LogRaw { // potentially change to From<...> for `LockedInBridge`.
        fn from(value: web3::types::Log) -> Self {
            Self { data: value.data.0.to_vec() }
        }
    }
}

pub struct Node<'a, I> 
    where I: std::marker::Send, LogRaw: From<I>
    
    {
    // events in the latest
    in_events_queue: Arc<Mutex<Vec<LockedInBridge>>>,

    // wrapper around the rpc client to interact with Soroban
    stellar_rpc: NodeStellarRpcClient<'a>,

    // ethereum listener object
    eth_listener: Box<dyn EventLogger<I>>,
}

// Define the trait
#[async_trait]
pub trait EventProcessor<I> {
    async fn bridge_in_contract_events_logger(&self, stream_item: Pin<Box<Next<'_, Pin<Box<dyn Stream<Item = I> + std::marker::Send>>>>>);
    async fn process_event(&self, event: LockedInBridge);
    async fn process_event_queue(&self);
}

// Implement the trait for Node
#[async_trait]
impl<'a, I: std::marker::Send> EventProcessor<I> for Node<'a, I>
    where LogRaw: From<I>
    
    {

    async fn bridge_in_contract_events_logger(&self, stream_item: Pin<Box<Next<'_, Pin<Box<dyn Stream<Item = I> + std::marker::Send>>>>>) {
        // TODO: Implement logger from an Ethereum contract
        // Reference implementation help: https://github.com/tomusdrw/rust-web3/blob/master/examples/contract_log_filter.rs

        
        // Implementation
        // let LogRaw { data } = stream_item.await.unwrap().into();
        // let event = LockedInBridge::deserialize_from_bytes(bytes);

        // Test implementation
        let address = stellar_strkey::ed25519::PublicKey::from_string("GAICVK2SYRLD7YFKD3D2TZGKAB6NH34VP4NW2ZYEEQHLLZOQIVC5VXEL").unwrap();
        let event = LockedInBridge::new(
            // preimage is `multichain-soroban-bridge`
            [219, 29, 40, 188, 191, 213, 181, 129, 93, 39, 8, 89, 7, 230, 165, 232, 72, 141, 15, 63, 124, 182, 125, 121, 2, 208, 54, 237, 51, 247, 70, 207], 
            
            address.0, 
            1000
        );
        
        self.in_events_queue.lock().unwrap().push(event);

        sleep(Duration::from_secs(2)).await;
        
        // drop lock
    }


    async fn process_event(&self, event: LockedInBridge) {
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

impl<'a, I: Send> Node<'a, I>
    where LogRaw: From<I>
    {
    /// Sets the initial parameters of the node and configurates the object.    
    pub fn new(node_secret: &'a str, out_contract: Bytes32, stellar_rpc_endpoint: &'a str, txload_function: &'a str, network_passphrase: &'a str, listener: impl EventLogger<I> + 'static) -> Self {
        let client = NodeStellarRpcClient::new(
            node_secret, 
            network_passphrase, 
            stellar_rpc_endpoint, 
            out_contract, 
            txload_function
        );
        
        Self { in_events_queue: Default::default(), stellar_rpc: client, eth_listener: Box::new(listener) }
    }

    pub async fn run(&self) {
        info!("[+] starting service");

        let mut topin = self.eth_listener.read_stream(Duration::from_secs(1)).await;
        
        loop {
            self.bridge_in_contract_events_logger(
                Box::pin(topin.next())
            ).await;
            self.process_event_queue().await;
        }
    }

    pub fn set_oracle_data(&self) {}
}
