use std::{time::Duration, sync::{Arc, RwLock}};

use futures::StreamExt;
use soroban_cortex_core::{SorobanEventsSteamConfig, EventsStream};
pub use soroban_cortex_core::Event;

pub struct EventsNode<'a> {
    executor: Box<dyn Fn(&Event) -> ()>,
    poll_interval: Duration,
    executed_ids: Arc<RwLock<Vec<String>>>,
    last_ledger_sequence_queried: Arc<RwLock<u32>>,
    events_stream: EventsStream<'a>,
}

impl<'a> EventsNode<'a> {
    pub fn new(
        executor: Box<dyn Fn(&Event) -> ()>,
        poll_interval: Duration,
        event_stream_config: SorobanEventsSteamConfig<'a>
    ) -> Self {
        let events_stream = EventsStream::new(event_stream_config);

        Self { 
            executor,
            poll_interval,
            executed_ids: Default::default(),
            last_ledger_sequence_queried: Default::default(),
            events_stream
        }
    }

    fn executor_ref(&self) -> &Box<dyn Fn(&Event) -> ()> {
        &self.executor
    }

    pub fn poll_interval(&self) -> Duration {
        self.poll_interval
    }

    pub fn streamer_obj_ref(&self) -> &EventsStream<'a> {
        &self.events_stream
    }

    pub fn executed_ids(&self) -> Arc<RwLock<Vec<String>>> {
        Arc::clone(&self.executed_ids) // Clone the Arc for the caller
    }

    pub fn put_executed(&self, id: String) {
        let mut executed_ids = self.executed_ids.write().unwrap();
        executed_ids.push(id);
    }
    
    pub fn clear_executed(&self) {
        let mut executed_ids = self.executed_ids.write().unwrap();
        executed_ids.clear();
    }

    pub fn last_ledger_sequence_queried(&self) -> u32 {
        *self.last_ledger_sequence_queried.read().unwrap()
    }

    pub fn set_last_ledger_sequence_queried(&self, sequence: u32) {
        *self.last_ledger_sequence_queried.write().unwrap() = sequence
    }

    /// Runs an event streamer node that
    /// executes the implementor-provided
    /// executor function on a vector of
    /// events streamed by 
    /// ['soroban_cortex_core::EventStream']
    /// according to the provided stream configurations
    /// (['soroban_cortex_core::SorobanEventsStreamConfig']).
    pub async fn run(&self) {
        let poll_interval = self.poll_interval();

        // Instantiating the stream.
        // Uses `soroban_cortex_core::EventsStream` to
        // get the stream object. 
        let stream = self
            .streamer_obj_ref()
            .stream(
                poll_interval
            );

        // Pinning on stack.
        futures::pin_mut!(stream);

        loop {
            // Vector of events from the latest ledger.
            // The events are filtered according to the
            // provided stream configuration ('soroban_cortex_core::SorobanEventsStreamConfig').
            let streamed = stream.next().await;
            
            if let Some(events) = streamed {                 
                for event in events {

                    // If the latest queried ledger is not the
                    // currently queried one then clear the executed
                    // ids vector since it's no longer needed.
                    if self.last_ledger_sequence_queried() < event.ledger.parse::<u32>().unwrap() {
                        self.clear_executed();
                        self.set_last_ledger_sequence_queried(event.ledger.parse().unwrap());
                    }

                    if !self.executed_ids().read().unwrap().contains(&event.id) {
                        self.executor_ref()(&event);
                        self.put_executed(event.id);
                    }

                }

            }
        }
    }
}


#[cfg(test)]

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

#[tokio::test]
async fn run_printer() {
    const LOG_CONTRACT: &str = "3fcfe95e48766b97a9f55a8bfa58fd79b62c7c4a23fb29738cd2e7cece27a281";
    const STARTING_LEDGER: u32 = 165564;
    const RPC_URL: &str = "https://rpc-futurenet.stellar.org:443/";
            
    let event_stream_config = SorobanEventsSteamConfig::new(
        RPC_URL, 
        STARTING_LEDGER, 
        LOG_CONTRACT, 
        None, 
        Duration::from_secs(3)
    );

    let node = EventsNode::new(Box::new(executors::printer_tracker), Duration::from_millis(4500), event_stream_config);
    node.run().await
}
