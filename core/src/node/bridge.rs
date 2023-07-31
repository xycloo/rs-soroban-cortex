use futures::{Stream, stream::Next, StreamExt};
use log::{info, debug};
use soroban_cli::rpc::Client;
use std::{pin::Pin};
use tokio::time::{sleep, Duration};
use async_trait::async_trait;

use crate::{
    rpc::{SorobanBridgeHelperRpc}, 
    
    messaging::{LockedInBridge, TryIntoMessage}, NodeConfiguration, EventLogger
};

use super::{EventProcessor, BridgeMessage};


// Bridge Messaging Node Implementation

#[async_trait]
impl<'a, I: std::marker::Send> EventProcessor<I> for BridgeMessage<'a, I>
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
        let tx = self.build_tx(payload).await;
        
        self.send_transaction(tx).await
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
