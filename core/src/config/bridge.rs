use ed25519_dalek::Keypair;

use crate::messaging::Bytes32;

pub enum ErrorMode {
    Strict,
    Permissive
} 
pub struct NodeConfiguration<'a> {
    max_calculated_fee: u32,
    error_logging_mode: ErrorMode,

    // Soroban
    pub rpc_endpoint: &'a str,
    pub aggregator_contract_id: Bytes32,
    pub txload_function: &'a str,
    pub key: Keypair,
    pub network_passphrase: &'a str,
}
