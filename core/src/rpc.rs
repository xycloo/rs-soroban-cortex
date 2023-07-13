//! Implements a wrapper type around the Soroban RPC client to send transactions and stream events on Soroban.

use async_trait::async_trait;
use soroban_cli::{rpc::Client};
use soroban_env_host::{xdr::{ScBytes, ScVal, Transaction}, I64Val};
use log::{error, info};
use crate::{SorobanEventsSteamConfig, utils::build_invoke_contract_tx, Node};

/// Client wrapper
pub struct NodeStellarRpcClient<'a> {
    config: SorobanEventsSteamConfig<'a>,
    
    /// RPC client
    pub client: Client,
}


#[async_trait]
pub trait SorobanRpc {
    async fn sequence_number(&self) -> i64;

    async fn build_tx(&self, payload: [u8; 80]) -> Transaction;

    async fn send_transaction(&self, tx: Transaction);
}


#[async_trait]
impl<'a, I> SorobanRpc for Node<'a, I>
    where I: Send 
    {
        /// Reads the node's account sequence number.
    async fn sequence_number(&self) -> i64 {
        let public_strkey = stellar_strkey::ed25519::PublicKey(self.config.node().key.public.to_bytes()).to_string();
        let account_details = self.stellar_rpc_client.get_account(&public_strkey).await.unwrap();
        account_details.seq_num.into()            
    }

    /// Builds the transaction used to broadcast the message.
    async fn build_tx(&self, payload: [u8; 80]) -> Transaction { // TODO: type alias for payload
        let config = &self.config.node();
        
        let complete_args = vec![
            ScVal::Bytes(ScBytes(config.aggregator_contract_id.try_into().unwrap())),
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

    /// Prepare and send the built transaction.
    /// This methods performs the appropriate checks before submitting to the Stellar Network:
    /// - check that the calculated fees do not exceed the maximum speficied when 
    async fn send_transaction(&self, tx: Transaction) {
        //let assembled = self.stellar_rpc_client.prepare_transaction(&tx, None).await.unwrap();
        //println!("{:?}", assembled);

        //let signed = utils::sign_transaction(&self.key, &assembled, &self.network_passphrase).unwrap();
        //println!("{}", signed.to_xdr_base64().unwrap());

        let config = &self.config.node();

        if let Err(error) = self.stellar_rpc_client.prepare_and_send_transaction(&tx, &config.key, config.network_passphrase, None).await {
            error!("submitting transaction to the Stellar network returns error {}", error);
        } else {
            info!("successfully transmitted message to Soroban")
        }
    }
}
