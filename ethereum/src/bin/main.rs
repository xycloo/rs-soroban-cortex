/*

Ethereum -> Soroban workflow:

0. write listener -> requires an eth node or instance to start an event0. write listener -> requires an eth node or instance to start an event
                     stream with web3-rs.
1. set up node
2. run node:
    loop {
        a. Listen for events.await
        b. Receive event -> add to queue
        c. Process events queue -> send message to soroban contract.await
    }

*/

use ethereum_bridge::{logger::EthEventLogger, conversions::LogWrap};
use soroban_cortex_core::{Node, EventLogger};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
     #[arg(short, long)]
    node_secret: String,

     #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let contract_address = [0;20];
    let listener = EthEventLogger::new(contract_address.as_slice());
}
