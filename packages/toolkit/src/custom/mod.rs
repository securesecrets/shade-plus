//! Custom types to be used across all types of contracts.

use cosmwasm_schema::cw_serde;

pub mod contract;
pub use contract::{Contract, RawContract};

#[cw_serde]
pub enum ResponseStatus {
    Success,
    Failure,
}