#[tokio::main]
async fn main() {
    // runs a node that prints events
    tests::stream_only::soroban_events_stream_hello_contract().await;
}