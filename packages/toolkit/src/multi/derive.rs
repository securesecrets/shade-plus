/// Used for creates a struct that implements the MultiTestable interface.
///
/// Needs the implementing package to have shade_protocol as a dependency with features.
///
/// First arg is the struct name that will implement the MultiTestable interface.
///
/// Second is the name of the package containing the contract module itself.
#[macro_export]
macro_rules! implement_testable {
    ($x:ident, $s:ident) => {
        use shade_toolkit::{App, ContractInstantiationInfo, ContractWrapper, MultiTestable};
        use cosmwasm_std::{Addr, ContractInfo};

        pub struct $x {
            info: ContractInfo,
        }

        impl MultiTestable for $x {
            fn store_contract(&self, app: &mut App) -> ContractInstantiationInfo {
                let contract = ContractWrapper::new_with_empty(
                    $s::contract::execute,
                    $s::contract::instantiate,
                    $s::contract::query,
                );
                app.store_code(Box::new(contract))
            }

            fn default() -> Self {
                let info = ContractInfo {
                    address: Addr::unchecked(""),
                    code_hash: String::default(),
                };
                $x { info }
            }
        }
    };
}

/// Used for creates a struct that implements the MultiTestable interface **(for contracts that implement the reply method)**
///
/// Needs the implementing package to have shade_protocol as a dependency with features.
///
/// First arg is the struct name that will implement the MultiTestable interface.
///
/// Second is the name of the package containing the contract module itself.
#[macro_export]
macro_rules! implement_testable_with_reply {
    ($x:ident, $s:ident) => {
        use shade_toolkit::{App, ContractInstantiationInfo, ContractWrapper, MultiTestable};
        use cosmwasm_std::{Addr, ContractInfo, Empty};

        pub struct $x {
            info: ContractInfo,
        }

        impl MultiTestable for $x {
            fn store_contract(&self, app: &mut App) -> ContractInstantiationInfo {
                let contract = ContractWrapper::new_with_empty(
                    $s::contract::execute,
                    $s::contract::instantiate,
                    $s::contract::query,
                )
                .with_reply($s::contract::reply);
                app.store_code(Box::new(contract))
            }

            fn default() -> Self {
                let info = ContractInfo {
                    address: Addr::unchecked(""),
                    code_hash: String::default(),
                };
                $x { info }
            }
        }
    };
}

/// Macro to generate the base for a multi testable contract.
#[macro_export]
macro_rules! create_testable_contract {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name(pub cosmwasm_std::ContractInfo);
        impl Into<shade_toolkit::Contract> for $name {
            fn into(self) -> shade_toolkit::Contract {
                shade_toolkit::Contract::new(self.0.address.as_str(), self.0.code_hash.as_str())
            }
        }
        impl Into<shade_toolkit::RawContract> for $name {
            fn into(self) -> shade_toolkit::RawContract {
                shade_toolkit::RawContract::new(self.0.address.as_str(), self.0.code_hash.as_str())
            }
        }
    };
}

/// Macro to generate the base for a multi testable contract with an admin user.
#[macro_export]
macro_rules! create_testable_contract_with_admin {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name(pub cosmwasm_std::ContractInfo, pub multi_test_helpers::User);
        impl Into<shade_toolkit::Contract> for $name {
            fn into(self) -> shade_toolkit::Contract {
                shade_toolkit::Contract::new(self.0.address.as_str(), self.0.code_hash.as_str())
            }
        }
        impl Into<shade_toolkit::RawContract> for $name {
            fn into(self) -> shade_toolkit::RawContract {
                shade_toolkit::RawContract::new(self.0.address.as_str(), self.0.code_hash.as_str())
            }
        }
    };
}
