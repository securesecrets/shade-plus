#[macro_export]
macro_rules! impl_into_contract {
    ($name:ident) => {
        impl Into<shade_toolkit::Contract> for $name {
            fn into(self) -> shade_toolkit::Contract {
                shade_toolkit::Contract {
                    address: self.0.address,
                    code_hash: self.0.code_hash,
                }
            }
        }
        impl Into<shade_toolkit::RawContract> for $name {
            fn into(self) -> shade_toolkit::RawContract {
                shade_toolkit::RawContract {
                    address: self.0.address.to_string(),
                    code_hash: self.0.code_hash,
                }
            }
        }
    };
}

/// Used for creates a struct that implements the MultiTestable interface.
///
/// Needs the implementing package to have shade_toolkit as a dependency with features.
///
/// First arg is the struct name that will implement the MultiTestable interface.
///
/// Second is the name of the package containing the contract module itself.
#[macro_export]
macro_rules! implement_harness {
    ($x:ident, $s:ident) => {
        use shade_toolkit::{App, ContractInstantiationInfo, ContractWrapper, MultiTestable};
        use cosmwasm_std::{Addr, ContractInfo};

        #[derive(Clone, Debug)]
        pub struct $x(ContractInfo);

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
                $x(info)
            }
        }
    };
}

#[macro_export]
macro_rules! implement_harness_with_default_admin {
    ($x:ident, $s:ident) => {
        use mulberry_multi_test_utils::traits::tester::User;
        use shade_toolkit::{App, ContractInstantiationInfo, ContractWrapper, MultiTestable};
        use cosmwasm_std::{Addr, ContractInfo};

        #[derive(Clone, Debug)]
        pub struct $x(ContractInfo, User);

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
                let user = User::default();
                $x(info, user)
            }
        }
    };
}

/// Used for creates a struct that implements the MultiTestable interface **(for contracts that implement the reply method)**
///
/// Needs the implementing package to have shade_toolkit as a dependency with features.
///
/// First arg is the struct name that will implement the MultiTestable interface.
///
/// Second is the name of the package containing the contract module itself.
#[macro_export]
macro_rules! implement_harness_with_reply {
    ($x:ident, $s:ident) => {
        use shade_toolkit::{App, ContractInstantiationInfo, ContractWrapper, MultiTestable};
        use cosmwasm_std::{Addr, ContractInfo};

        #[derive(Clone, Debug)]
        pub struct $x(ContractInfo);

        impl MultiTestable for $x {
            fn store_contract(&self, app: &mut App) -> ContractInstantiationInfo {
                let contract = ContractWrapper::new_with_empty(
                    $s::contract::execute,
                    $s::contract::instantiate,
                    $s::contract::query,
                ).with_reply($s::contract::reply);
                app.store_code(Box::new(contract))
            }

            fn default() -> Self {
                let info = ContractInfo {
                    address: Addr::unchecked(""),
                    code_hash: String::default(),
                };
                $x(info)
            }
        }
    };
}
