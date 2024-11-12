#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct ImmutableCounter;
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contractimpl]
impl ImmutableCounter {
    /// Example constructor
    pub fn __constructor(env: Env, counter: u32) {
        env.storage().persistent().set(&COUNTER, &counter);
    }
    /// Counter value
    pub fn counter(env: Env) -> u32 {
        env.storage().persistent().get(&COUNTER).unwrap()
    }
}

mod test;
