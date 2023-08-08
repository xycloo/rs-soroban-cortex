# Soroban Events Streaming
#### Quickly spin up events streaming services for Soroban contracts.

This crate is built on top of `soroban-cortex-core` and is designed to allow for a customizable implementation of an event streaming service for Soroban smart contracts.

The goal is to provide a highly customizable node that is saves lots of time for implementors to deal with actual event streaming. 


# Getting Started

The crate is under development and its code is not stable nor audited, and it's missing many features that are mandatory for a production-ready service. However, you can already try it out:

```rust
use std::{time::Duration};

use futures::StreamExt;
use soroban_cortex_core::{SorobanEventsSteamConfig, EventsStream};
pub use soroban_cortex_core::Event;


mod executors {
    use soroban_cortex_core::Event;

    pub fn printer_tracker(event: &Event) {
        let topics = &event.topic;

        if topics.eq(&vec!["AAAADgAAAAVoZWxsbwAAAA==".to_string()]) {
            let id = &event.id;
            
            println!(
                "\n[{}] New hello event streamed!",
                id
            );
        }
    
    }
}

#[tokio::main]
async fn main() {
    const LOG_CONTRACT: &str = "3fcfe95e48766b97a9f55a8bfa58fd79b62c7c4a23fb29738cd2e7cece27a281";
    const STARTING_LEDGER: u32 = 165564;
    const RPC_URL: &str = "https://rpc-futurenet.stellar.org:443/";
            
    let event_stream_config = SorobanEventsSteamConfig::new(
        RPC_URL, 
        STARTING_LEDGER, 
        LOG_CONTRACT, 
        None, 
        Duration::from_secs(3) // ignore this field.
                               // as we previosuly said
                               // the whole codebase needs a bit
                               // of restructuring.
    );

    let node = EventsNode::new(Box::new(executors::printer_tracker), Duration::from_millis(4500), event_stream_config);
    node.run().await
}

```

The above code streams events for contract `3fcfe95e48766b97a9f55a8bfa58fd79b62c7c4a23fb29738cd2e7cece27a281`, a [simple log contract](https://github.com/xycloo/rs-soroban-cortex/tree/main/test_contracts/simple_log).
