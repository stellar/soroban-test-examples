#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};
use this_one_signs::SigningContract;

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let user = Address::generate(&env);
    let contract_id = env.register(SignedContract, ());
    let signing_contract_id = env.register(SigningContract, ());
    let client = SignedContractClient::new(&env, &contract_id);

    let words = client.hello(&String::from_str(&env, "Dev"), &user, &signing_contract_id);
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "Hello"),
            String::from_str(&env, "Dev"),
        ]
    );
}
