use crate::{SorobanConfig, NodeConfiguration};

pub mod generic;
pub mod soroban;


pub struct Config<'a> {
    soroban: SorobanConfig<'a>,
    node: NodeConfiguration
}

impl<'a> Config<'a> {
    pub fn new(soroban: SorobanConfig<'a>, node: NodeConfiguration) -> Self {
        Self { 
            soroban, 
            node 
        }
    }

    // todo: add getters for the two configs
}
