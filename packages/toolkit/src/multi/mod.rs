//! Helper functions and structs that make it easier to test contracts.
pub mod derive;

use std::panic;
use core::{
    panic::AssertUnwindSafe,
    fmt::Debug,
};

use cosmwasm_std::{Timestamp, Addr, ContractInfo, StdResult, Coin};
use serde::de::DeserializeOwned;
pub use shade_multi_test::*;

use crate::{Query, InstantiateCallback, ExecuteCallback};

/// Trait for making integration with multi-test easier.
pub trait MultiTestable {
    fn store_contract(&self, app: &mut App) -> ContractInstantiationInfo;
    fn default() -> Self;
}

pub type AppResult = AnyResult<AppResponse>;

pub trait Suite {
    fn app(&mut self) -> &mut App;
    fn set_time(&mut self, new_time: u64) {
        self.app().update_block(|b| b.time = Timestamp::from_seconds(new_time));
    }
    fn set_block(&mut self, new_block: u64) {
        self.app().update_block(|b| b.height = new_block);
    }
    fn increment_blocks(&mut self, blocks: u64) {
        self.app().update_block(|b| b.height += blocks);
    }
    fn fast_forward(&mut self, seconds: u64) {
        self.app().update_block(|b| b.time = b.time.plus_seconds(seconds));
    }
    fn rewind(&mut self, seconds: u64) {
        self.app().update_block(|b| b.time = b.time.minus_seconds(seconds));
    }
    /// Assert that the result of code that is unwind safe is an error.
    fn unwind_err(hook: impl FnOnce()) {
        let res = panic::catch_unwind(AssertUnwindSafe(hook));
        let is_err = res.is_err();
        assert!(is_err);
    }

    fn assert_error(res: AppResult, expected: impl ToString) {
        assert_eq!(res.unwrap_err().root_cause().to_string(), expected.to_string());
    }

    /// Assert that two vectors are equal, ignoring order.
    fn assert_equal_vecs<T>(a: &[T], b: &[T])
    where
        T: Ord + Debug + Clone,
    {
        let mut a = a.to_vec();
        let mut b = b.to_vec();
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }
}

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

