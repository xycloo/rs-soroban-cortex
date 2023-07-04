use futures::{Stream, StreamExt, stream::Next, TryStreamExt, TryStream};
use log::{info, debug};
use soroban_cli::rpc::{EventStart, EventType, Event};
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
        fn stream(&self, poll_interval:Duration) -> impl Stream<Item = std::vec::Vec<soroban_cli::rpc::Event>> + '_{
            let client = &self.stellar_rpc;
            
            let current_ledger = self.stellar.starting_ledger;
            

            futures::stream::unfold(current_ledger, move |current_ledger: u32| async move {
                tokio::time::sleep(poll_interval);
            
                let event_start = EventStart::Ledger(current_ledger);
                
                let contract_id_string = stellar_strkey::Contract(self.stellar.contract_id);
                let items = client.client.get_events(
                    event_start, 
                    Some(EventType::Contract), 
                    &[contract_id_string.to_string()], 
                    self.stellar.topics, 
                    None
                ).await.unwrap(); // TODO: error handling.

                Some((items.events, items.latest_ledger))
            })
        }
    }
