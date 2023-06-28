use ed25519_dalek::Keypair;
use soroban_cli::{commands::config::secret::Secret, rpc::Client, utils};
use soroban_env_host::xdr::{
    self, HostFunction, HostFunctionArgs, InvokeHostFunctionOp,
    Memo, MuxedAccount, OperationBody, Preconditions, ScBytes, ScVal,
    ScVec, SequenceNumber, Transaction, TransactionExt, Uint256, VecM, WriteXdr,
};
use log::{error, info};
use crate::node::Bytes32;

fn build_invoke_contract_tx(
    parameters: ScVec,
    sequence: i64,
    fee: u32,
    key: &ed25519_dalek::Keypair,
) -> Transaction {
    let op = xdr::Operation {
        source_account: None,
        body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
            functions: vec![HostFunction {
                args: HostFunctionArgs::InvokeContract(parameters),
                auth: VecM::default(),
            }]
            .try_into().unwrap(),
        }),
    };

    Transaction {
        source_account: MuxedAccount::Ed25519(Uint256(key.public.to_bytes())),
        fee,
        seq_num: SequenceNumber(sequence),
        cond: Preconditions::None,
        memo: Memo::None,
        operations: vec![op].try_into().unwrap(),
        ext: TransactionExt::V0,
    }
}

pub struct NodeStellarRpcClient<'a> {
    contract_id: Bytes32,
    txload_function: &'a str,
    key: Keypair,
    client: Client,
    network_passphrase: &'a str,
}


impl<'a> NodeStellarRpcClient<'a> {
    pub fn new(secret: &str, network_passphrase: &'a str, base_url: &str, contract_id: Bytes32, txload_function: &'a str) -> Self {
        let key = secret
        .parse::<Secret>()
        .unwrap()
        .key_pair(None)
        .unwrap();

        let client = Client::new(base_url).unwrap();

        Self { contract_id, txload_function, key, client, network_passphrase }
    }

    pub async fn sequence_number(&self) -> i64 {
        let public_strkey = stellar_strkey::ed25519::PublicKey(self.key.public.to_bytes()).to_string();
        let account_details = self.client.get_account(&public_strkey).await.unwrap();
        account_details.seq_num.into()            
    }

    pub async fn build_tx(&self, payload: [u8; 80]) -> Transaction { // TODO: type alias for payload
        let complete_args = vec![
            ScVal::Bytes(ScBytes(self.contract_id.try_into().unwrap())),
            ScVal::Symbol(
                self.txload_function
                    .try_into()
                    .unwrap(),
            ),
            ScVal::Bytes(ScBytes(payload.try_into().unwrap())) // serialized tx object [hash, recipient, amount]
        ];
        let sequence = self.sequence_number().await + 1;
        
        let tx = build_invoke_contract_tx(complete_args.try_into().unwrap(), sequence, 100, &self.key);
        tx
    }

    pub async fn send_transaction(&self, tx: Transaction) {
        //let assembled = self.client.prepare_transaction(&tx, None).await.unwrap();
        //println!("{:?}", assembled);

        //let signed = utils::sign_transaction(&self.key, &assembled, &self.network_passphrase).unwrap();
        //println!("{}", signed.to_xdr_base64().unwrap());

        if let Err(error) = self.client.prepare_and_send_transaction(&tx, &self.key, &self.network_passphrase, None).await {
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



