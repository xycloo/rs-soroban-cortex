use std::time::Duration;

use ed25519_dalek::Keypair;

use crate::messaging::Bytes32;

pub struct SorobanEventsSteamConfig<'a > {
    pub rpc_endpoint: &'a str,
    pub starting_ledger: u32,
    pub topics: Option<&'a [String]>,
    pub contract_id: &'a str,
    pub poll_interval: Duration
}

impl<'a> SorobanEventsSteamConfig<'a> {
    pub fn new(rpc_endpoint: &'a str, starting_ledger: u32, contract_id: &'a str, topics: Option<&'a [String]>, poll_interval: Duration) -> Self {
        Self { 
            rpc_endpoint,
            starting_ledger,
            contract_id,
            topics,
            poll_interval
        }
    }
}
