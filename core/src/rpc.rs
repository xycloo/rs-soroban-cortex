use soroban_cli::{rpc::Client};
use soroban_env_host::xdr::{ScBytes, ScVal, Transaction};
use log::{error, info};
use crate::{config::soroban::SorobanConfig, utils::build_invoke_contract_tx};

pub struct NodeStellarRpcClient<'a> {
    config: SorobanConfig<'a>,
    client: Client,
}


impl<'a> NodeStellarRpcClient<'a> {
    pub fn new(config: SorobanConfig<'a>) -> Self {
        // todo: helper to create key from secret string
        // let key = secret
        // .parse::<Secret>()
        // .unwrap()
        // .key_pair(None)
        // .unwrap();

        let client = Client::new(config.rpc_endpoint).unwrap(); // todo: possibly remove rpc_endpoint from config and add it as param

        Self { config, client }
    }

    pub async fn sequence_number(&self) -> i64 {
        let public_strkey = stellar_strkey::ed25519::PublicKey(self.config.key.public.to_bytes()).to_string();
        let account_details = self.client.get_account(&public_strkey).await.unwrap();
        account_details.seq_num.into()            
    }

    pub async fn build_tx(&self, payload: [u8; 80]) -> Transaction { // TODO: type alias for payload
        let config = &self.config;
        
        let complete_args = vec![
            ScVal::Bytes(ScBytes(config.contract_id.try_into().unwrap())),
            ScVal::Symbol(
                config.txload_function
                    .try_into()
                    .unwrap(),
            ),
            ScVal::Bytes(ScBytes(payload.try_into().unwrap())) // serialized tx object [hash, recipient, amount]
        ];
        let sequence = self.sequence_number().await + 1;
        
        let tx = build_invoke_contract_tx(complete_args.try_into().unwrap(), sequence, 100, &config.key);
        tx
    }

    pub async fn send_transaction(&self, tx: Transaction) {
        //let assembled = self.client.prepare_transaction(&tx, None).await.unwrap();
        //println!("{:?}", assembled);

        //let signed = utils::sign_transaction(&self.key, &assembled, &self.network_passphrase).unwrap();
        //println!("{}", signed.to_xdr_base64().unwrap());

        let config = &self.config;

        if let Err(error) = self.client.prepare_and_send_transaction(&tx, &config.key, config.network_passphrase, None).await {
            error!("submitting transaction to the Stellar network returns error {}", error);
        } else {
            info!("successfully transmitted message to Soroban")
        }
    }
}

/* 
#[test]
fn test_tx() {
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
      };

    let tx = run_rpc_tx("https://rpc-futurenet.stellar.org:443/soroban/rpc", "SC7PJSRS6JKKHG7W3U6LHF7V3TXAEYS34GAB3EK5FWVS6DU4SEHBM3I2");
    println!("{:?}", aw!(tx));

    
}
*/



