#[tokio::main]
async fn main() {
    // runs a node that prints events
    tests::stream_only_packaged::soroban_events_stream_hello_contract().await;

    // runs a node that prints events (without using `run`, rather by implementing the run logic itself).
    // tests::unpackaged::soroban_events_stream_hello_contract().await;
}
