use ethereum::{logger::EthEventLogger, conversions::LogWrap};
use multichain_core::{Node, EventLogger};

fn main() {
    let listener = EthEventLogger::new(contract_address);
    let node = Node::<LogWrap>::new(
        node_secret, 
        out_contract, 
        stellar_rpc_endpoint, 
        txload_function, 
        network_passphrase, 
        listener
    );
}
