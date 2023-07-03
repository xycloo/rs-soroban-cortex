//! Node implementation and logic.
//! TODO: write documentation for setting up and running a node.
//! 

/*

Node settings:

- node stellar secret [Node doesn't require a secret 
                       for the initiating chain. The only
                       component that will possibly need to
                       execute operation on the initiating
                       chain must be configured by the implementor
                       anyway (like `EventLogger`).]
- stellar contract address
- stellar rpc endpoint
- stellar network passphrase
- function name of stellar contract fn to load oracle data
- maximum calculated fee
- errors logging mode:
    * strict
    * permissive

*By the implementor:*
- listener obect
- Option<callback operation executor> [optionally the implementor can
                                       setup an object that reads soroban
                                       events and exeuctes a tx on the 
                                       initiating chain]

*/

use futures::{Stream, StreamExt, stream::Next};
use log::{info, debug};
use std::{sync::{Arc, Mutex}, pin::Pin};
use tokio::time::{sleep, Duration};
use async_trait::async_trait;

use crate::{
    rpc::NodeStellarRpcClient, 
    config::soroban::SorobanConfig, 
    messaging::{LockedInBridge, EventLogger, TryIntoMessage}
};



/// Node object.
/// [`I`] is the type that is streamed from the events logger the implementor provides.
/// ['I'] must implement [`TryIntoMessage`] for it to be serialized to bytes.
pub struct Node<'a, I> 
    where I: std::marker::Send
    {

    /// Queued events.
    in_events_queue: Arc<Mutex<Vec<LockedInBridge>>>,

    // TODO: maybe remove this wrapper and make it a trait would make it easier
    // for error logs reporting and 

    /// Wrapper around the rpc client to interact with Soroban.
    stellar_rpc: NodeStellarRpcClient<'a>,

    // Ethereum event logger, supplied by implementor.
    eth_listener: Box<dyn EventLogger<I>>,

}

/// Describes the behaviour of processing an events stream.
#[async_trait]
pub trait EventProcessor<I> {
    /// Awaits for the stream to be updated with a new item, when it is, deserialize the event and push it to the queue.
    async fn read_stream_next(&self, stream_item: Pin<Box<Next<'_, Pin<Box<dyn Stream<Item = I> + std::marker::Send>>>>>);
    
    /// Processes a given event by sending it as message to the Soroban contract in the form of a contract invocation.
    async fn process_event(&self, event: LockedInBridge);
    
    /// Processes the events queue.
    async fn process_event_queue(&self);
}

#[async_trait]
impl<'a, I: std::marker::Send> EventProcessor<I> for Node<'a, I>
    where I: TryIntoMessage
    
    {

    async fn read_stream_next(&self, stream_item: Pin<Box<Next<'_, Pin<Box<dyn Stream<Item = I> + std::marker::Send>>>>>) {
        // TODO: Implement logger from an Ethereum contract
        // Reference implementation help: https://github.com/tomusdrw/rust-web3/blob/master/examples/contract_log_filter.rs

        
        // Implementation

        // TODO: error checking
        // leave up to the implementor if node is
        // strict or permissive:
        // strict: any event that can't be converted
        // i.e an event from another action will be reported
        // in the errors log.
        // permissive: any event that can't be converted
        // will be ignored.

        // the current implementation below assumes a permissive mode.
        if let Ok(bytes) = stream_item.await.unwrap().try_into() {
            debug!("event is being processed.");
            
            let event = LockedInBridge::deserialize_from_bytes(bytes); // TODO: probably remove byte serialization/deserialization.
            self.in_events_queue.lock().unwrap().push(event);
        } else {
            debug!("event is not deserializable into message.")
        };

        

        // Test implementation
        
        let address = stellar_strkey::ed25519::PublicKey::from_string("GAICVK2SYRLD7YFKD3D2TZGKAB6NH34VP4NW2ZYEEQHLLZOQIVC5VXEL").unwrap();
        let event = LockedInBridge::new(
            // hash preimage is `multichain-soroban-bridge`
            [219, 29, 40, 188, 191, 213, 181, 129, 93, 39, 8, 89, 7, 230, 165, 232, 72, 141, 15, 63, 124, 182, 125, 121, 2, 208, 54, 237, 51, 247, 70, 207], 
            
            address.0, 
            1000
        );
        self.in_events_queue.lock().unwrap().push(event);
        sleep(Duration::from_secs(2)).await;

        // TODO: remove this test implementation
    }


    async fn process_event(&self, event: LockedInBridge) {
        let payload = event.serialize_to_bytes(); // TODO: probably remove byte serialization/deserialization.
        
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
    where I: TryIntoMessage
    {

    /// Sets the initial parameters of the node and configurates the object.    
    pub fn new(soroban_config: SorobanConfig<'a>, listener: impl EventLogger<I> + 'static) -> Self {
        let client = NodeStellarRpcClient::new(
            soroban_config
        );
        
        Self { in_events_queue: Default::default(), stellar_rpc: client, eth_listener: Box::new(listener) }
    }

    /// Runs the node.
    pub async fn run(&self) {
        info!("[+] starting service");

        let mut topin = self.eth_listener.read_stream(Duration::from_secs(1)).await;
        
        loop {
            self.read_stream_next(
                Box::pin(topin.next())
            ).await;
            self.process_event_queue().await;
        }
    }

}
