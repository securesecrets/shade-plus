use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Addr};
use query_authentication::permit::Permit;

use crate::{InstantiateCallback, Contract, ExecuteCallback, ResponseStatus, Query};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_auth: Contract,
    pub prng_seed: Binary,
}

impl InstantiateCallback for InstantiateMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum ContractStatus {
    Default,
    DisablePermit,
    DisableVK,
    DisableAll,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetAdminAuth {
        admin: Contract,
        padding: Option<String>,
    },
    SetRunState {
        state: ContractStatus,
        padding: Option<String>,
    },

    SetViewingKey {
        key: String,
        padding: Option<String>,
    },
    CreateViewingKey {
        entropy: String,
        padding: Option<String>,
    },

    BlockPermitKey {
        key: String,
        padding: Option<String>,
    },
}

impl ExecuteCallback for ExecuteMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum ExecuteAnswer {
    SetAdminAuth { status: ResponseStatus },
    SetRunState { status: ResponseStatus },
    SetViewingKey { status: ResponseStatus },
    CreateViewingKey { key: String },
    BlockPermitKey { status: ResponseStatus },
}

pub type QueryPermit = Permit<PermitData>;

#[remain::sorted]
#[cw_serde]
pub struct PermitData {
    pub data: Binary,
    pub key: String,
}

#[cw_serde]
pub enum QueryMsg {
    Config {},

    ValidateViewingKey { user: Addr, key: String },
    ValidatePermit { permit: QueryPermit },
}

impl Query for QueryMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum QueryAnswer {
    Config {
        admin: Contract,
        state: ContractStatus,
    },
    ValidateViewingKey {
        is_valid: bool,
    },
    ValidatePermit {
        user: Addr,
        is_revoked: bool,
    },
}
