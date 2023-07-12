use bytes::Bytes;
use futures::{Stream, StreamExt, stream::Next};
use log::{info, debug};
use soroban_cli::rpc::Client;
use std::{sync::{Arc, Mutex}, pin::Pin};
use tokio::time::{sleep, Duration};
use async_trait::async_trait;

use crate::{
    rpc::NodeStellarRpcClient, 
    config::{soroban::SorobanConfig, Config}, 
    messaging::{LockedInBridge, EventLogger, TryIntoMessage, Bytes32}, NodeError, NodeConfiguration
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
    pub stellar_rpc_client: Client,

    /// Configurations
    pub config: Config<'a>,

    /// Ethereum event logger, supplied by implementor.
    pub eth_listener: Option<Box<dyn EventLogger<I>>>,

}


// TODO: rename new(s) to new_bridge and new_soroban_stream

#[cfg(feature = "full")]
impl <'a, I: Send> Node<'a, I> 
    where I: TryIntoMessage 
    
    {
    
    /// Sets the initial parameters of the node and configurates the object.    
    pub fn new(soroban_config: SorobanConfig<'a>, node_config: NodeConfiguration<'a>, listener: impl EventLogger<I> + 'static) -> Self {
        let stellar_rpc_client = Client::new(soroban_config.rpc_endpoint).unwrap(); // todo: error handling

        let config = Config::new(Some(soroban_config), Some(node_config));

        Self { 
            in_events_queue: Default::default(), 
            stellar_rpc_client, 
            config, 
            eth_listener: Some(Box::new(listener)) 
        }
    }

    /// Runs the node.
    pub async fn run(&self) {
        info!("[+] starting service");

        let mut topin = self.eth_listener.as_ref().unwrap().read_stream(Duration::from_secs(1)).await;
        
        loop {
            self.read_stream_next(
                Box::pin(topin.next())
            ).await;

            self.process_event_queue().await;

        };
    }


    }

#[cfg(feature = "stream_only")]
impl <'a> Node<'a, ()>     
    {
    
    /// Sets the initial parameters of the node and configurates the object.    
    pub fn new(soroban_config: SorobanConfig<'a>) -> Self {
        let stellar_rpc_client = Client::new(soroban_config.rpc_endpoint).unwrap(); // todo: error handling

        let config = Config::new(Some(soroban_config), None);

        Self { 
            in_events_queue: Default::default(), 
            stellar_rpc_client, 
            config, 
            eth_listener: None
        }
    }

    /// Runs the node.
    pub async fn run(&self) {
        info!("[+] starting service");

        let soroban_stream = self.stream(self.config.soroban().poll_interval);
        futures::pin_mut!(soroban_stream);

        loop {
            println!("{:?}", soroban_stream.next().await)
        };
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
