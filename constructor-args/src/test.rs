#![cfg(test)]

use super::*;
use soroban_sdk::{vec, xdr::ScVal, Env};

#[test]
fn constructor_args() {
    let env = Env::default();
    let contract_id = env.register(ImmutableCounter, vec![&env, ScVal::U32(42)]);
    let client = ImmutableCounterClient::new(&env, &contract_id);
    assert_eq!(42u32, client.counter());
}
