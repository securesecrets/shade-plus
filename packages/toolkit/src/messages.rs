#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "testing")]
use crate::multi_test::{AnyResult, App, AppResponse, Executor, MultiTestable};
use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, ContractInfo, CosmosMsg, QuerierWrapper, QueryRequest, Response,
    StdResult, WasmMsg, WasmQuery,
};
use serde::{de::DeserializeOwned, Serialize};

/// Take a Vec<u8> and pad it up to a multiple of `block_size`, using spaces at the end.
pub fn space_pad(message: &mut Vec<u8>, block_size: usize) -> &mut Vec<u8> {
    let len = message.len();
    let surplus = len % block_size;
    if surplus == 0 {
        return message;
    }

    let missing = block_size - surplus;
    message.reserve(missing);
    message.extend(std::iter::repeat(b' ').take(missing));
    message
}

/// Pad the data and logs in a `Response` to the block size, with spaces.
// The big `where` clause is based on the `where` clause of `Response`.
// Users don't need to care about it as the type `T` has a default, and will
// always be known in the context of the caller.
pub fn pad_execute_result(response: StdResult<Response>, block_size: usize) -> StdResult<Response> {
    response.map(|mut response| {
        response.data = response.data.map(|mut data| {
            space_pad(&mut data.0, block_size);
            data
        });
        for log in &mut response.attributes {
            // Safety: These two are safe because we know the characters that
            // `space_pad` appends are valid UTF-8
            unsafe { space_pad(log.key.as_mut_vec(), block_size) };
            unsafe { space_pad(log.value.as_mut_vec(), block_size) };
        }
        response
    })
}

/// Pad a `StdResult<Binary>` with spaces
pub fn pad_query_result(response: StdResult<Binary>, block_size: usize) -> StdResult<Binary> {
    response.map(|mut response| {
        space_pad(&mut response.0, block_size);
        response
    })
}

/// A trait marking types that define the instantiation message of a contract
///
/// This trait requires specifying a padding block size and provides a method to create the
/// CosmosMsg used to instantiate a contract
pub trait InstantiateCallback: Serialize {
    /// pad the message to blocks of this size
    const BLOCK_SIZE: usize;

    /// Returns StdResult<CosmosMsg>
    ///
    /// Tries to convert the instance of the implementing type to a CosmosMsg that will trigger the
    /// instantiation of a contract.  The BLOCK_SIZE specified in the implementation is used when
    /// padding the message
    ///
    /// # Arguments
    ///
    /// * `label` - String holding the label for the new contract instance
    /// * `code_id` - code ID of the contract to be instantiated
    /// * `callback_code_hash` - String holding the code hash of the contract to be instantiated
    /// * `send_amount` - Optional Uint128 amount of native coin to send with instantiation message
    fn to_cosmos_msg(
        &self,
        label: impl Into<String>,
        code_id: u64,
        code_hash: impl Into<String>,
        funds: Vec<Coin>,
    ) -> StdResult<CosmosMsg> {
        let mut msg = to_binary(self)?;
        // can not have 0 block size
        let padding = if Self::BLOCK_SIZE == 0 {
            1
        } else {
            Self::BLOCK_SIZE
        };
        space_pad(&mut msg.0, padding);
        let init = WasmMsg::Instantiate {
            code_id,
            code_hash: code_hash.into(),
            msg,
            label: label.into(),
            funds,
            admin: None,
        };
        Ok(init.into())
    }

    fn to_cosmos_msg_with_admin(
        &self,
        label: impl Into<String>,
        code_id: u64,
        code_hash: impl Into<String>,
        funds: Vec<Coin>,
        admin: Option<String>,
    ) -> StdResult<CosmosMsg> {
        let mut msg = to_binary(self)?;
        // can not have 0 block size
        let padding = if Self::BLOCK_SIZE == 0 {
            1
        } else {
            Self::BLOCK_SIZE
        };
        space_pad(&mut msg.0, padding);
        let init = WasmMsg::Instantiate {
            code_id,
            code_hash: code_hash.into(),
            msg,
            label: label.into(),
            funds,
            admin,
        };
        Ok(init.into())
    }

    /// Returns ContractInfo
    ///
    /// Tries to instantiate a contract into the multi test app.
    ///
    /// # Arguments
    ///
    /// * `testable` - a struct implementing the MultiTestable trait
    /// * `app` - mutable reference to multi test app
    /// * `sender` - user performing init
    /// * `label` - label used to reference this contract
    /// * `send_funds` - any funds sent with this init
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "testing")]
    fn test_init(
        &self,
        testable: impl MultiTestable,
        app: &mut App,
        sender: impl Into<String>,
        label: impl Into<String>,
        send_funds: &[Coin],
    ) -> AnyResult<ContractInfo> {
        let stored_code = testable.store_contract(app);
        app.instantiate_contract(
            stored_code,
            Addr::unchecked(sender),
            &self,
            send_funds,
            label,
            None,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "testing")]
    fn test_init_with_admin(
        &self,
        testable: impl MultiTestable,
        app: &mut App,
        sender: impl Into<String>,
        label: impl Into<String>,
        send_funds: &[Coin],
        admin: Option<String>,
    ) -> AnyResult<ContractInfo> {
        let stored_code = testable.store_contract(app);
        app.instantiate_contract(
            stored_code,
            Addr::unchecked(sender),
            &self,
            send_funds,
            label,
            admin,
        )
    }
}

/// A trait marking types that define the handle message(s) of a contract
///
/// This trait requires specifying a padding block size and provides a method to create the
/// CosmosMsg used to execute a handle method of a contract
pub trait ExecuteCallback: Serialize {
    /// pad the message to blocks of this size
    const BLOCK_SIZE: usize;

    /// Returns StdResult<CosmosMsg>
    ///
    /// Tries to convert the instance of the implementing type to a CosmosMsg that will trigger a
    /// handle function of a contract.  The BLOCK_SIZE specified in the implementation is used when
    /// padding the message
    ///
    /// # Arguments
    ///
    /// * `callback_code_hash` - String holding the code hash of the contract to be executed
    /// * `contract_addr` - address of the contract being called
    /// * `send_amount` - Optional Uint128 amount of native coin to send with the handle message
    fn to_cosmos_msg(
        &self,
        contract: &(impl Into<ContractInfo> + Clone),
        funds: Vec<Coin>,
    ) -> StdResult<CosmosMsg> {
        let mut msg = to_binary(self)?;
        // can not have 0 block size
        let padding = if Self::BLOCK_SIZE == 0 {
            1
        } else {
            Self::BLOCK_SIZE
        };
        let contract: ContractInfo = contract.clone().into();
        space_pad(&mut msg.0, padding);
        let execute = WasmMsg::Execute {
            msg,
            contract_addr: contract.address.to_string(),
            code_hash: contract.code_hash,
            funds,
        };
        Ok(execute.into())
    }

    /// Returns AnyResult<AppResponse>
    ///
    /// Tries to execute a message on a contract in the multi-test App.
    ///
    /// # Arguments
    ///
    /// * `contract` - ContractInfo of an existing contract on the multi-test App
    /// * `app` - a mutable reference to the multi-test App
    /// * `sender` - the user executing this message in the test env
    /// * `send_funds` - any funds transferred with this exec
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "testing")]
    fn test_exec(
        &self,
        contract: &(impl Into<ContractInfo> + Clone),
        app: &mut App,
        sender: impl Into<String>,
        send_funds: &[Coin],
    ) -> AnyResult<AppResponse>
    where
        Self: Serialize + std::fmt::Debug,
    {
        let contract: ContractInfo = contract.clone().into();
        app.execute_contract(Addr::unchecked(sender), &contract, &self, send_funds)
    }
}

/// A trait marking types that define the query message(s) of a contract
///
/// This trait requires specifying a padding block size and provides a method to query a contract
pub trait Query: Serialize {
    /// pad the message to blocks of this size
    const BLOCK_SIZE: usize;

    /// Returns StdResult<T>, where T is the type defining the query response
    ///
    /// Tries to query a contract and deserialize the query response.  The BLOCK_SIZE specified in the
    /// implementation is used when padding the message
    ///
    /// # Arguments
    ///
    /// * `querier` - a reference to the Querier dependency of the querying contract
    /// * `callback_code_hash` - String holding the code hash of the contract to be queried
    /// * `contract_addr` - address of the contract being queried
    fn query<T: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
        contract: &(impl Into<ContractInfo> + Clone),
    ) -> StdResult<T> {
        let mut msg = to_binary(self)?;
        // can not have 0 block size
        let padding = if Self::BLOCK_SIZE == 0 {
            1
        } else {
            Self::BLOCK_SIZE
        };
        space_pad(&mut msg.0, padding);
        let contract: ContractInfo = contract.clone().into();
        querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: contract.address.to_string(),
            msg,
            code_hash: contract.code_hash,
        }))
    }

    /// Returns StdResult<T>, where T is the type defining the query response
    ///
    /// Tries to query a contract in the multi-test App.
    ///
    /// # Arguments
    ///
    /// * `info` - contract info of instantiated contract
    /// * `app` - a reference to the multi-test App
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "testing")]
    fn test_query<T: DeserializeOwned>(
        &self,
        info: &(impl Into<ContractInfo> + Clone),
        app: &App,
    ) -> StdResult<T> {
        let mut msg = to_binary(self)?;
        // can not have 0 block size
        let padding = if Self::BLOCK_SIZE == 0 {
            1
        } else {
            Self::BLOCK_SIZE
        };
        let info: ContractInfo = info.clone().into();
        space_pad(&mut msg.0, padding);
        app.wrap().query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: info.address.to_string(),
            msg,
            code_hash: info.code_hash.clone(),
        }))
    }
}
