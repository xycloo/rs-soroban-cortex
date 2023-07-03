use futures::{Stream, StreamExt, stream::Next};
use log::{info, debug};
use soroban_cli::rpc::EventStart;
use std::{sync::{Arc, Mutex}, pin::Pin};
use tokio::time::{sleep, Duration};
use async_trait::async_trait;

use crate::{
    rpc::NodeStellarRpcClient, 
    config::soroban::SorobanConfig, 
    messaging::{LockedInBridge, EventLogger, TryIntoMessage}, Node, NodeError
};

use super::{SorobanEvent};

impl<'a, I: Send> Node<'a, I>
    where I: TryIntoMessage 
    
    {
        fn stream(&self, poll_interval:Duration) -> impl Stream<Item = Result<SorobanEvent, NodeError>> {
            let client = &self.stellar_rpc;
            
            // EventStart

            futures::stream::unfold(client, move |state| async move {
                // client.client.get_events(, event_type, contract_ids, topics, limit)
            })
        }
    }
