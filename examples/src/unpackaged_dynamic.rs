use std::collections::HashSet;
use std::sync::Arc;
use futures::pin_mut;
use soroban_cortex_core::{SorobanEventsSteamConfig, EventsStream};
use tokio::time::{Duration, interval};
use tokio::sync::{broadcast, Mutex, mpsc};
use futures::stream::StreamExt;

pub async fn soroban_events_stream_hello_contract() {
    let rpc_url = "https://rpc-futurenet.stellar.org:443/";

    // Create a channel to receive contract IDs from another part of your service
    let (contract_sender, mut contract_receiver) = mpsc::unbounded_channel::<String>();

    // Set up a HashSet to keep track of active contract IDs
    let active_contracts = Arc::new(Mutex::new(HashSet::new()));

    // Clone references for use in the tasks
    let active_contracts_clone = active_contracts.clone();
    let contract_sender_clone = contract_sender.clone();

    // Spawn an asynchronous task to handle incoming contract IDs
    tokio::spawn(async move {
        while let Some(new_contract_id) = contract_receiver.recv().await {
            // Add the new contract ID to the active contracts set
            let mut active_contracts = active_contracts_clone.lock().await;
            active_contracts.insert(new_contract_id);
        }
    });

    // Spawn an asynchronous task to manage the event stream
    tokio::spawn(async move {
        loop {
            // Create a new event stream with the current active contract IDs
            let active_contracts = active_contracts.lock().await;
            let active_contract_ids: Vec<String> = active_contracts.iter().cloned().collect();
            drop(active_contracts); // Release the lock before creating the stream
            let soroban_config = SorobanEventsSteamConfig::new(rpc_url, 78780, active_contract_ids.last().unwrap(), None, Duration::from_secs(3));
            let node = EventsStream::new(soroban_config);
            let mut stream = node.stream(Duration::from_secs(4));

            pin_mut!(stream);

            // Process events from the stream
            while let Some(event) = stream.next().await {
                println!("{:?}", event);
            }

            // Sleep before checking for more events
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // Example: Dynamically add a new contract ID (replace "new_contract_id" with the actual ID)
    contract_sender.send("new_contract_id".to_string()).unwrap();

    // Keep the main thread alive
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    
}