use soroban_cli::rpc::Client;

use crate::SorobanEventsSteamConfig;


pub struct StateWatcher<'a> {
    pub stellar_rpc_client: Client,
    pub config: SorobanEventsSteamConfig<'a>
}

impl<'a> StateWatcher<'a> {
    //pub fn 
}
