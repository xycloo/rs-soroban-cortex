use ed25519_dalek::Keypair;

use crate::messaging::Bytes32;

pub struct SorobanConfig<'a > {
    pub rpc_endpoint: &'a str,
    pub starting_ledger: u32,
    pub topics: Option<&'a [String]>,
    pub contract_id: &'a str,

    // the features below may be better off in generic/bridge
    /* 
    pub aggregator_contract_id: Bytes32,
    pub txload_function: &'a str,
    pub key: Keypair,
    pub network_passphrase: &'a str,
    */
}

// contract_id: Bytes32, function_name: &'a str, network_passphrase: &'a str, key: Keypair

impl<'a> SorobanConfig<'a> {
    pub fn new(rpc_endpoint: &'a str, starting_ledger: u32, contract_id: &'a str, topics: Option<&'a [String]>) -> Self {
        Self { 
            rpc_endpoint,
            starting_ledger,
            contract_id,
            topics,
        }
    }
}
