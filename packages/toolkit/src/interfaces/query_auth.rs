use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Addr, QuerierWrapper, StdResult, ContractInfo, StdError, from_binary};
use query_authentication::permit::Permit;
use serde::de::DeserializeOwned;

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

pub struct PermitAuthentication<T: DeserializeOwned> {
    pub sender: Addr,
    pub revoked: bool,
    pub data: T
}

/// Authenticates a permit through query auth and returns the result without deserializing the binary.
pub fn authenticate_arbitrary_permit<U: Into<ContractInfo> + Clone>(
    permit: QueryPermit,
    querier: &QuerierWrapper,
    authenticator: &U,
) -> StdResult<PermitAuthentication<Binary>> {
    let res: QueryAnswer =
        QueryMsg::ValidatePermit { permit: permit.clone() }.query(querier, authenticator)?;

    let sender: Addr;
    let revoked: bool;

    match res {
        QueryAnswer::ValidatePermit { user, is_revoked } => {
            sender = user;
            revoked = is_revoked;
        }
        _ => return Err(StdError::generic_err("Unexpected response from query auth.")),
    }

    Ok(PermitAuthentication {
        sender,
        revoked,
        data: permit.params.data,
    })
}

pub fn authenticate_permit<T: DeserializeOwned, U: Into<ContractInfo> + Clone>(
    permit: QueryPermit,
    querier: &QuerierWrapper,
    authenticator: &U,
) -> StdResult<PermitAuthentication<T>> {
    let res: QueryAnswer = QueryMsg::ValidatePermit { permit: permit.clone() }
        .query(querier, authenticator)?;

    let sender: Addr;
    let revoked: bool;

    match res {
        QueryAnswer::ValidatePermit { user, is_revoked } => {
            sender = user;
            revoked = is_revoked;
        }
        _ => return Err(StdError::generic_err("Wrong query response")),
    }

    Ok(PermitAuthentication {
        sender,
        revoked,
        data: from_binary(&permit.params.data)?
    })
}

pub fn authenticate_vk<U: Into<ContractInfo> + Clone>(
    address: Addr,
    key: String,
    querier: &QuerierWrapper,
    authenticator: &U
) -> StdResult<bool> {
    let res: QueryAnswer = QueryMsg::ValidateViewingKey {
        user: address,
        key,
    }.query(querier, authenticator)?;

    match res {
        QueryAnswer::ValidateViewingKey { is_valid } => {
            Ok(is_valid)
        }
        _ => Err(StdError::generic_err("Unauthorized")),
    }
}