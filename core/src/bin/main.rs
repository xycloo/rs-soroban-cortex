use multichain_core::{Node, EventLogger, SorobanConfig, utils::build_key_from_secret};
use env_logger;

#[tokio::main]
async fn main() {
    let eth_listener = multichain_core::chains_test_types::ethereum::MyListener::new(&[0;20]);

    let rpc_endpoint = "http://localhost:8000/soroban/rpc";
    let secret = "SC7PJSRS6JKKHG7W3U6LHF7V3TXAEYS34GAB3EK5FWVS6DU4SEHBM3I2";
    let contract_id = [235, 212, 101, 5, 30, 144, 131, 210, 126, 200, 204, 44, 180, 132, 16, 104, 235, 75, 115, 26, 211, 167, 169, 157, 31, 77, 233, 247, 11, 6, 55, 79];
    let network_passphrase = "Standalone Network ; February 2017";
    let function_name = "oracle_set";

    let soroban_config = SorobanConfig::new(
        rpc_endpoint, 
        contract_id, 
        function_name, 
        network_passphrase, 
        build_key_from_secret(secret)
    );

    env_logger::init();
    node.run().await;
}