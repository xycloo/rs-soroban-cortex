use stellar_xdr::next::{ReadXdr, WriteXdr};

pub const LOG_CONTRACT: &str = "3fcfe95e48766b97a9f55a8bfa58fd79b62c7c4a23fb29738cd2e7cece27a281";
pub const STARTING_LEDGER: u32 = 165564;

pub mod stream_only_packaged {
    use std::time::Duration;

    use soroban_cortex_core::{SorobanEventsSteamConfig, EventsStream};

    pub async fn soroban_events_stream_hello_contract() {
        let rpc_url = "https://rpc-futurenet.stellar.org:443/";
        let contract_id = "116668071f9c9669bf451851a960c9d55a20964bbd2438d08adb59f21b6ffe6b";
                
        let soroban_config = SorobanEventsSteamConfig::new(rpc_url, 78780, contract_id, None, Duration::from_secs(3));

        let node = EventsStream::new(soroban_config);

        node.run().await
    }
}



pub mod unpackaged;
pub mod unpackaged_dynamic;

#[test]
fn build_xdr() {
    //let wasm_hash = stellar_xdr::next::Hash::from_xdr_base64("N6IjTWyKonr2NcLiaSg/EG1xYYuROOo08JWtcRDYgJc=");
    
    //println!("{:?}", wasm_hash.unwrap().to_string());

    let i = stellar_xdr::next::ScVal::LedgerKeyContractInstance;
    println!("{}", i.to_xdr_base64().unwrap());

}
