use ed25519_dalek::Keypair;
use soroban_cli::commands::config::secret::Secret;
use soroban_env_host::xdr::{
    self, HostFunction, InvokeHostFunctionOp,
    Memo, MuxedAccount, OperationBody, Preconditions,
    ScVec, SequenceNumber, Transaction, TransactionExt, Uint256, VecM, Operation,
};

pub fn build_key_from_secret(secret: &str) -> Keypair {
    secret
        .parse::<Secret>()
        .unwrap()
        .key_pair(None)
        .unwrap()
}

pub fn build_invoke_contract_tx(
    parameters: ScVec,
    sequence: i64,
    fee: u32,
    key: &ed25519_dalek::Keypair,
) -> Transaction {
    let op = Operation {
        source_account: None,
        body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
            host_function: HostFunction::InvokeContract(parameters),
            auth: VecM::default(),
        }),
    };
    
    Transaction {
        source_account: MuxedAccount::Ed25519(Uint256(key.public.to_bytes())),
        fee,
        seq_num: SequenceNumber(sequence),
        cond: Preconditions::None,
        memo: Memo::None,
        operations: vec![op].try_into().unwrap_or_else(|_| panic!("invalid parameters")),
        ext: TransactionExt::V0,
    }
}

