#![no_std]

use soroban_sdk::{contractimpl, Env, BytesN, Address};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn oracle_set(env: Env, payload: BytesN<80>) {}
}
