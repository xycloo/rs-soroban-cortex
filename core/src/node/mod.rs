use bytes::Bytes;
use futures::{Stream, StreamExt, stream::Next};
use log::{info, debug};
use soroban_cli::rpc::Client;
use std::{sync::{Arc, Mutex}, pin::Pin};
use tokio::time::{Duration};
use async_trait::async_trait;

use crate::{
    config::Config,
    SorobanEventsSteamConfig, 
    messaging::{LockedInBridge, EventLogger, TryIntoMessage, Bytes32}, NodeConfiguration
};

pub struct BridgeMessage<'a, I> 
    where I: std::marker::Send
    {

    /// Queued events.
    in_events_queue: Arc<Mutex<Vec<LockedInBridge>>>,

    pub stellar_rpc_client: Client,

    /// Configuration
    pub config: NodeConfiguration<'a>,

    /// Event logger of the chain initiating the swap, supplied by implementor.
    pub initiator_listener: Box<dyn EventLogger<I>>,
    
    }


pub struct EventsStream<'a> {
    pub stellar_rpc_client: Client,
    pub config: SorobanEventsSteamConfig<'a>
}


// TODO: rename new(s) to new_bridge and new_soroban_stream

#[cfg(feature = "packaged")]
impl <'a, I: Send> BridgeMessage<'a, I> 
    where I: TryIntoMessage 
    
    {
    
    /// Sets the initial parameters of the node and configurates the object.    
    pub fn new(node_config: NodeConfiguration<'a>, listener: impl EventLogger<I> + 'static) -> Self {
        let stellar_rpc_client = Client::new(node_config.rpc_endpoint).unwrap(); // todo: error handling

        
        Self { 
            in_events_queue: Default::default(), 
            stellar_rpc_client, 
            config: node_config, 
            initiator_listener: Box::new(listener) 
        }
    }

    /// Runs the node.
    pub async fn run(&self) {
        info!("[+] starting service");

        let mut pinned = self.initiator_listener.as_ref().read_stream(
            Duration::from_secs(1)
        ).await;
        
        loop {
            self.read_stream_next(
                Box::pin(pinned.next())
            ).await;

            self.process_event_queue().await;

        };
    }
    }


#[cfg(feature = "packaged")]
impl <'a> EventsStream<'a>  
    {
    
    /// Sets the initial parameters of the node and configurates the object.    
    pub fn new(config: SorobanEventsSteamConfig<'a>) -> Self {
        let stellar_rpc_client = Client::new(config.rpc_endpoint).unwrap(); // todo: error handling

        Self { 
            stellar_rpc_client, 
            config, 
        }
    }

    /// Runs the node.
    pub async fn run(&self) {
        info!("[+] starting service");

        let soroban_stream = self.stream(self.config.poll_interval);
        futures::pin_mut!(soroban_stream);

        loop {
            println!("{:?}", soroban_stream.next().await)
        };
    }


    }



/// Describes the behaviour of processing an events stream.
// #[cfg(feature = "bridge")]
#[async_trait]
pub trait EventProcessor<I> {
    /// Awaits for the stream to be updated with a new item, when it is, deserialize the event and push it to the queue.
    async fn read_stream_next(&self, stream_item: Pin<Box<Next<'_, Pin<Box<dyn Stream<Item = I> + std::marker::Send>>>>>);
    
    /// Processes a given event by sending it as message to the Soroban contract in the form of a contract invocation.
    async fn process_event(&self, event: LockedInBridge);
    
    /// Processes the events queue.
    async fn process_event_queue(&self);
}


// #[cfg(feature = "soroban_events_stream")]
pub struct SorobanEvent {
    contract_id: Bytes32,
    topics: Vec<Bytes>,
    data: Bytes
}


// #[cfg(feature = "bridge")]
mod bridge;

// #[cfg(feature = "soroban_events_stream")]
mod soroban;

// mod soroban_state_watcher;
