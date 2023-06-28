use core::node::Node;
use env_logger;

#[tokio::main]
async fn main() {
    let node = Node::new(
        "SC7PJSRS6JKKHG7W3U6LHF7V3TXAEYS34GAB3EK5FWVS6DU4SEHBM3I2", 
        [0;32], 
        [235, 212, 101, 5, 30, 144, 131, 210, 126, 200, 204, 44, 180, 132, 16, 104, 235, 75, 115, 26, 211, 167, 169, 157, 31, 77, 233, 247, 11, 6, 55, 79], 
        "http://localhost:8000/soroban/rpc", 
        "oracle_set", 
        "Standalone Network ; February 2017"
    );

    env_logger::init();
    node.run().await;
}