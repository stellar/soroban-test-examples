use soroban_sdk::{contractevent, Address};

#[contractevent(data_format = "single-value")]
pub struct SetAdmin {
    #[topic]
    pub admin: Address,
    pub new_admin: Address,
}
