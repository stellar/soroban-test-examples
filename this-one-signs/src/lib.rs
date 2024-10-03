#![no_std]
use soroban_sdk::{
    auth::Context, auth::CustomAccountInterface, contract, contracterror, contractimpl,
    crypto::Hash, vec, xdr::Signature, Env, String, Vec,
};

#[contract]
pub struct SigningContract;

#[contractimpl]
impl SigningContract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SigningError {
    FakeError = 1,
}

impl CustomAccountInterface for SigningContract {
    type Error = SigningError;
    type Signature = Vec<Signature>;

    #[allow(non_snake_case)]
    fn __check_auth(
        _env: Env,
        _signature_payload: Hash<32>,
        _signatures: Vec<Signature>,
        _auth_context: Vec<Context>,
    ) -> Result<(), SigningError> {
        Ok(())
    }
}

mod test;
