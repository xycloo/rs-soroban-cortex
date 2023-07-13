use crate::{SorobanEventsSteamConfig, NodeConfiguration, Node};

pub mod bridge;
pub mod soroban_events_stream;


pub struct Config<'a> {
    soroban: Option<SorobanEventsSteamConfig<'a>>,
    node: Option<NodeConfiguration<'a>>
}

impl<'a> Config<'a> {
    pub fn new(soroban: Option<SorobanEventsSteamConfig<'a>>, node: Option<NodeConfiguration<'a>>) -> Self {
        
        Self { 
            soroban, 
            node
        }
    }

    pub fn soroban(&self) -> &SorobanEventsSteamConfig {
        self.soroban.as_ref().unwrap_or_else(|| panic!("Requesting a field that should not be used for your features, check your feature configurations."))

    }

    pub fn node(&self) -> &NodeConfiguration {
        self.node.as_ref().unwrap_or_else(|| panic!("Requesting a field that should not be used for your features, check your feature configurations."))
       
    }
}
