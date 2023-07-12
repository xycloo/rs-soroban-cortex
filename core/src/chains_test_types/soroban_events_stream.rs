use crate::Node;

/* 
#[test]
#[cfg(feature = "stream_only")]
fn get_stream() {
    use crate::SorobanConfig;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
      };

    let rpc_url = "https://rpc-futurenet.stellar.org:443/soroban/rpc";
    let node_secret = "SC7PJSRS6JKKHG7W3U6LHF7V3TXAEYS34GAB3EK5FWVS6DU4SEHBM3I2";
    let contract_id = [0;32];
    let topics = &["hello"];
    
    let soroban_config = SorobanConfig::new(rpc_url, 751355, );

    Node::new(soroban_config);
}
*/