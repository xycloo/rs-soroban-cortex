use ed25519_dalek::Keypair;

use crate::messaging::Bytes32;

pub struct SorobanConfig<'a > {
    pub rpc_endpoint: &'a str,
    pub starting_ledger: u32,
    pub contract_id: Bytes32,
    pub topics: &'a [String],
    pub txload_function: &'a str,
    pub key: Keypair,
    pub network_passphrase: &'a str,
    }

impl<'a> SorobanConfig<'a> {
    pub fn new(rpc_endpoint: &'a str, starting_ledger: u32, contract_id: Bytes32, topics: &'a [String], function_name: &'a str, network_passphrase: &'a str, key: Keypair) -> Self {
        Self { 
            rpc_endpoint,
            starting_ledger,
            contract_id,
            topics, 
            txload_function: function_name, 
            key, 
            network_passphrase 
        }
    }
}
