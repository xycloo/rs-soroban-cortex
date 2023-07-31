use std::time::Duration;

use futures::StreamExt;
use soroban_cortex_core::{SorobanEventsSteamConfig, EventsStream};

use crate::{STARTING_LEDGER, LOG_CONTRACT};

pub async fn soroban_events_stream_hello_contract() {
    let rpc_url = "https://rpc-futurenet.stellar.org:443/";
            
    let soroban_config = SorobanEventsSteamConfig::new(
        rpc_url, 
        STARTING_LEDGER, 
        LOG_CONTRACT, 
        None, 
        Duration::from_secs(3)
    );

    let node = EventsStream::new(soroban_config);

    let stream = node.stream(Duration::from_secs(4));
    futures::pin_mut!(stream);
    
    loop {
        println!("{:?}", stream.next().await)
    }
}
