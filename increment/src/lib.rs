#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Initialize the counter with a value
    pub fn __constructor(env: Env, counter: u32) {
        env.storage().instance().set(&COUNTER, &counter);
    }

    /// Increment internal counter, return the value
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&COUNTER)
            .expect("counter not set!");
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);

        // The contract instance will be bumped to have a lifetime of at least 100 ledgers if the current expiration lifetime at most 50.
        // If the lifetime is already more than 100 ledgers, this is a no-op. Otherwise,
        // the lifetime is extended to 100 ledgers. This lifetime bump includes the contract
        // instance itself and all entries in storage().instance(), i.e, COUNTER.
        env.storage().instance().extend_ttl(50, 100);

        // Return the count to the caller.
        count
    }

    /// Get current counter value
    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap()
    }
}

mod test;
