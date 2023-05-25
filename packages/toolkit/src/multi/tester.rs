use crate::multi::{
    AnyResult, App, AppResult, ExecuteCallback, InstantiateCallback, MultiTestable, Query,
};
use cosmwasm_std::{Addr, Coin, ContractInfo, StdResult};
use serde::de::DeserializeOwned;

pub trait Tester: Clone {
    fn addr(&self) -> Addr;
    fn str(&self) -> String {
        self.addr().to_string()
    }
    fn query<T: DeserializeOwned>(
        app: &App,
        msg: &impl Query,
        contract: &ContractInfo,
    ) -> StdResult<T> {
        msg.test_query(contract, app)
    }
    fn init(
        &self,
        app: &mut App,
        msg: &impl InstantiateCallback,
        testable: impl MultiTestable,
        label: &str,
    ) -> AnyResult<ContractInfo> {
        msg.test_init(testable, app, &self.str(), label, &[])
    }
    fn init_with_funds(
        &self,
        app: &mut App,
        msg: &impl InstantiateCallback,
        testable: impl MultiTestable,
        label: &str,
        send_funds: &[Coin],
    ) -> AnyResult<ContractInfo> {
        msg.test_init(testable, app, &self.str(), label, send_funds)
    }
    fn exec(
        &self,
        app: &mut App,
        msg: &(impl ExecuteCallback + std::fmt::Debug),
        contract: &ContractInfo,
    ) -> AppResult {
        msg.test_exec(contract, app, &self.str(), &[])
    }
    fn exec_with_funds(
        &self,
        app: &mut App,
        msg: &(impl ExecuteCallback + std::fmt::Debug),
        contract: &ContractInfo,
        send_funds: &[Coin],
    ) -> AppResult {
        msg.test_exec(contract, app, &self.str(), send_funds)
    }
}

#[derive(Clone)]
pub struct User {
    pub address: Addr,
}

impl Tester for User {
    fn addr(&self) -> Addr {
        self.address.clone()
    }
}

impl User {
    pub fn new(address: impl Into<String>) -> Self {
        let address = Addr::unchecked(address);
        User { address }
    }
}

impl From<Addr> for User {
    fn from(a: Addr) -> Self {
        User { address: a }
    }
}

impl<'a> From<&'a Addr> for User {
    fn from(a: &'a Addr) -> Self {
        User { address: a.clone() }
    }
}

impl From<String> for User {
    fn from(s: String) -> Self {
        User {
            address: Addr::unchecked(s),
        }
    }
}

impl From<&str> for User {
    fn from(s: &str) -> Self {
        User {
            address: Addr::unchecked(s),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Addr> for User {
    fn into(self) -> Addr {
        self.address
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for User {
    fn into(self) -> String {
        self.address.to_string()
    }
}
