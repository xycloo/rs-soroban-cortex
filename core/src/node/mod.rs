use bytes::Bytes;
use futures::{Stream, StreamExt, stream::Next};
use log::{info, debug};
use std::{sync::{Arc, Mutex}, pin::Pin};
use tokio::time::{sleep, Duration};
use async_trait::async_trait;

use crate::{
    rpc::NodeStellarRpcClient, 
    config::soroban::SorobanConfig, 
    messaging::{LockedInBridge, EventLogger, TryIntoMessage, Bytes32}, NodeError
};


/// Default node object.
/// This kind of node is designed to broadcast messages from one generic chain to soroban
/// while listening for callback events on Soroban to execute transactions on the generic chain.
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

    //#[cfg(feature = "soroban_events_stream")]
    stellar: SorobanConfig<'a>,

    // Ethereum event logger, supplied by implementor.
    #[cfg(feature = "bridge")]
    eth_listener: Box<dyn EventLogger<I>>,

}

impl<'a, I: Send> Node<'a, I>
    where I: TryIntoMessage

    {

    /// Sets the initial parameters of the node and configurates the object.    
    pub fn new(soroban_config: SorobanConfig<'a>, listener: impl EventLogger<I> + 'static) -> Self {
        let client = NodeStellarRpcClient::new(
            &soroban_config
        );
        
        Self { in_events_queue: Default::default(), stellar_rpc: client, stellar: soroban_config, eth_listener: Box::new(listener) }
    }

    /// Runs the node.
    pub async fn run(&self) {
        info!("[+] starting service");

        let mut topin = self.eth_listener.read_stream(Duration::from_secs(1)).await;
        
        loop {
            if cfg!(feature = "bridge") {
                self.read_stream_next(
                    Box::pin(topin.next())
                ).await;

                self.process_event_queue().await;
            }

            if cfg!(feature = "soroban_events_stream") {

            }
        }
    }

}



/// Describes the behaviour of processing an events stream.
#[cfg(feature = "bridge")]
#[async_trait]
pub trait EventProcessor<I> {
    /// Awaits for the stream to be updated with a new item, when it is, deserialize the event and push it to the queue.
    async fn read_stream_next(&self, stream_item: Pin<Box<Next<'_, Pin<Box<dyn Stream<Item = I> + std::marker::Send>>>>>);
    
    /// Processes a given event by sending it as message to the Soroban contract in the form of a contract invocation.
    async fn process_event(&self, event: LockedInBridge);
    
    /// Processes the events queue.
    async fn process_event_queue(&self);
}


#[cfg(feature = "soroban_events_stream")]
pub struct SorobanEvent {
    contract_id: Bytes32,
    topics: Vec<Bytes>,
    data: Bytes
}


#[cfg(feature = "bridge")]
mod generic;

#[cfg(feature = "soroban_events_stream")]
mod soroban_stream;
