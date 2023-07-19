
pub mod stream_only {
    use std::time::Duration;

    use soroban_cortex_core::{SorobanEventsSteamConfig, Node};

    pub async fn soroban_events_stream_hello_contract() {
        let rpc_url = "https://rpc-futurenet.stellar.org:443/";
        let contract_id = "116668071f9c9669bf451851a960c9d55a20964bbd2438d08adb59f21b6ffe6b";
                
        let soroban_config = SorobanEventsSteamConfig::new(rpc_url, 78780, contract_id, None, Duration::from_secs(3));

        let node = Node::<'_, ()>::new(soroban_config);

        node.run().await
    }
}
