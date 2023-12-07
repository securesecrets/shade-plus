use anyhow::bail;
use cosmwasm_std::{
    Addr, Api, Binary, BlockInfo, CustomQuery, Empty, IbcMsg, IbcQuery, Querier, Storage,
};
use schemars::JsonSchema;

use crate::{AnyResult, AppResponse, CosmosRouter, Module};

pub trait Ibc: Module<ExecT = IbcMsg, QueryT = IbcQuery, SudoT = IbcSudo> {}

#[derive(Clone, std::fmt::Debug, PartialEq, JsonSchema)]
pub enum IbcSudo {
    CheckBalance { channel_id: String, address: String },
}

#[derive(Default)]
pub struct IbcKeeper {}

impl IbcKeeper {
    pub fn new() -> Self {
        IbcKeeper {}
    }
}

impl Ibc for IbcKeeper {}

impl Module for IbcKeeper {
    type ExecT = IbcMsg;
    type QueryT = IbcQuery;
    type SudoT = IbcSudo;

    fn execute<ExecC, QueryC: CustomQuery>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        msg: IbcMsg,
    ) -> AnyResult<AppResponse> {
        match msg {
            m => bail!("Unsupported IBC message: {:?}", m),
            IbcMsg::Transfer {
                channel_id,
                to_address,
                amount,
                timeout,
                memo,
            } => todo!(),
            IbcMsg::SendPacket {
                channel_id,
                data,
                timeout,
            } => todo!(),
            IbcMsg::CloseChannel { channel_id } => todo!(),
            _ => todo!(),
        }
    }

    fn sudo<ExecC, QueryC: CustomQuery>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        msg: IbcSudo,
    ) -> AnyResult<AppResponse> {
        match msg {
            IbcSudo::CheckBalance {
                channel_id,
                address,
            } => todo!(),
            m => bail!("Unsupported IBC sudo message: {:?}", m),
        }
    }

    fn query(
        &self,
        api: &dyn Api,
        storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: IbcQuery,
    ) -> AnyResult<Binary> {
        match request {
            IbcQuery::PortId {} => todo!(),
            IbcQuery::ListChannels { port_id } => todo!(),
            IbcQuery::Channel {
                channel_id,
                port_id,
            } => todo!(),
            q => bail!("Unsupported staking query: {:?}", q),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{Addr, Binary, Empty, IbcMsg, IbcQuery};

    use crate::test_helpers::contracts::stargate::{contract, ExecMsg};
    use crate::{App, AppBuilder, AppResponse, Executor, Module};

    use crate::Ibc;

    use super::IbcSudo;

    struct AcceptingModule;

    impl Module for AcceptingModule {
        type ExecT = IbcMsg;
        type QueryT = IbcQuery;
        type SudoT = IbcSudo;

        fn execute<ExecC, QueryC>(
            &self,
            _api: &dyn cosmwasm_std::Api,
            _storage: &mut dyn cosmwasm_std::Storage,
            _router: &dyn crate::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
            _block: &cosmwasm_std::BlockInfo,
            _sender: cosmwasm_std::Addr,
            _msg: Self::ExecT,
        ) -> anyhow::Result<crate::AppResponse>
        where
            ExecC: std::fmt::Debug
                + Clone
                + PartialEq
                + schemars::JsonSchema
                + serde::de::DeserializeOwned
                + 'static,
            QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
        {
            Ok(AppResponse::default())
        }

        fn sudo<ExecC, QueryC>(
            &self,
            _api: &dyn cosmwasm_std::Api,
            _storage: &mut dyn cosmwasm_std::Storage,
            _router: &dyn crate::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
            _block: &cosmwasm_std::BlockInfo,
            _msg: Self::SudoT,
        ) -> anyhow::Result<crate::AppResponse>
        where
            ExecC: std::fmt::Debug
                + Clone
                + PartialEq
                + schemars::JsonSchema
                + serde::de::DeserializeOwned
                + 'static,
            QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
        {
            Ok(AppResponse::default())
        }

        fn query(
            &self,
            _api: &dyn cosmwasm_std::Api,
            _storage: &dyn cosmwasm_std::Storage,
            _querier: &dyn cosmwasm_std::Querier,
            _block: &cosmwasm_std::BlockInfo,
            _request: Self::QueryT,
        ) -> anyhow::Result<cosmwasm_std::Binary> {
            Ok(Binary::default())
        }
    }

    impl Ibc for AcceptingModule {}

    #[test]
    fn default_ibc() {
        let mut app = App::default();
        let code = app.store_code(contract());
        let contract = app
            .instantiate_contract(
                code,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "contract",
                None,
            )
            .unwrap();

        app.execute_contract(Addr::unchecked("owner"), &contract, &ExecMsg::Ibc {}, &[])
            .unwrap_err();
    }

    #[test]
    fn subsituting_ibc() {
        let mut app = AppBuilder::new()
            .with_ibc(AcceptingModule)
            .build(|_, _, _| ());
        let code = app.store_code(contract());
        let contract = app
            .instantiate_contract(
                code,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "contract",
                None,
            )
            .unwrap();

        app.execute_contract(Addr::unchecked("owner"), &contract, &ExecMsg::Ibc {}, &[])
            .unwrap();
    }
}
