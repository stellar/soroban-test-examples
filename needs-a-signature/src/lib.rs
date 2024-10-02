#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Address, Env, String, Vec};

#[contract]
pub struct SignedContract;

#[contractimpl]
impl SignedContract {
    pub fn hello(
        env: Env,
        to: String,
        user_signer: Address,
        contract_signer: Address,
    ) -> Vec<String> {
        user_signer.require_auth();
        contract_signer.require_auth();
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
