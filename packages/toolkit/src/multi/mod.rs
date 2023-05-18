//! Helper functions and structs that make it easier to test contracts.
mod tester;

pub mod derive;

pub use shade_multi_test::*;
pub use tester::*;

/// Trait for making integration with multi-test easier.
pub trait MultiTestable {
    fn store_contract(&self, app: &mut App) -> ContractInstantiationInfo;
    fn default() -> Self;
}

pub type AppResult = AnyResult<AppResponse>;
