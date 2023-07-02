use ed25519_dalek::Keypair;

use crate::node::Bytes32;

pub struct SorobanConfig<'a > {
    pub rpc_endpoint: &'a str,
    pub contract_id: Bytes32,
    pub txload_function: &'a str,
    pub key: Keypair,
    pub network_passphrase: &'a str,
}

impl<'a> SorobanConfig<'a> {
    pub fn new(rpc_endpoint: &'a str, contract_id: Bytes32, function_name: &'a str, network_passphrase: &'a str, key: Keypair) -> Self {
        Self { 
            rpc_endpoint,
            contract_id, 
            txload_function: function_name, 
            key, 
            network_passphrase 
        }
    }
}
