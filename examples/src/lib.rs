
pub mod stream_only {
    use std::time::Duration;

    use soroban_cortex_core::{SorobanEventsSteamConfig, Node};

    pub async fn soroban_events_stream_hello_contract() {
        let rpc_url = "https://rpc-futurenet.stellar.org:443/soroban/rpc";
        let contract_id = "bda1498d887f46c30cef17a046e2d96febad8a7e400d1335891f415ce5559577";
                
        let soroban_config = SorobanEventsSteamConfig::new(rpc_url, 60729, contract_id, None, Duration::from_secs(3));

        let node = Node::<'_, ()>::new(soroban_config);

        node.run().await
    }
}
