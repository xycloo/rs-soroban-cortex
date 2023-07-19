#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        env.events().publish(("hello", ), ("hello", "event"))
    }

    pub fn bump(env: Env) {
        env.storage().instance().bump(100000000);
    }
}
