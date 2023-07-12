#![no_std]

use soroban_sdk::{contractimpl, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        env.events().publish(("hello", ), ("hello", "event"))
    }
}
