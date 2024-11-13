#![cfg(test)]

use super::{IncrementContract, IncrementContractClient};
use soroban_sdk::{testutils::Logs, vec, xdr::ScVal, Env};

extern crate std;

#[test]
fn construct_get_increment() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, vec![&env, ScVal::U32(42)]);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.get(), 42);
    assert_eq!(client.increment(), 43);
    assert_eq!(client.increment(), 44);
    assert_eq!(client.increment(), 45);

    std::println!("{}", env.logs().all().join("\n"));
}
