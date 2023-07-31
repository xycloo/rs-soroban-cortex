use chrono::Duration;

pub struct LedgerBumpAmounts {
    pub instance: u32,
    pub persistent: u32
}

pub struct SorobanStateWatcher<'a > {
    pub rpc_endpoint: &'a str,
    pub contract_ids: &'a [&'a str],
    pub ledger_bump_amounts: LedgerBumpAmounts,
    pub filter: Option<Box<dyn Fn()>>,
    pub poll_interval: Duration
}

impl<'a> SorobanStateWatcher<'a> {
    pub fn new(rpc_endpoint: &'a str, contract_ids: &'a [&'a str], poll_interval: Duration, ledger_bump_amounts: LedgerBumpAmounts, filter: Option<Box<dyn Fn()>> ) -> Self {
        Self { 
            rpc_endpoint,
            contract_ids,
            ledger_bump_amounts,
            poll_interval,
            filter,
        }
    }
}

