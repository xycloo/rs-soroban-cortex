use std::time::Duration;

use futures::StreamExt;
use soroban_cortex_core::{SorobanEventsSteamConfig, EventsStream};

pub async fn soroban_events_stream_hello_contract() {
    let rpc_url = "https://rpc-futurenet.stellar.org:443/";
    let contract_id = "116668071f9c9669bf451851a960c9d55a20964bbd2438d08adb59f21b6ffe6b";
            
    let soroban_config = SorobanEventsSteamConfig::new(rpc_url, 78780, contract_id, None, Duration::from_secs(3));

    let node = EventsStream::new(soroban_config);

    let stream = node.stream(Duration::from_secs(4));
    futures::pin_mut!(stream);
    
    loop {
        println!("{:?}", stream.next().await)
    }
}
