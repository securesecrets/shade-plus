use cosmwasm_std::{Empty, GovMsg};

use crate::{
    ibc::IbcKeeper,
    staking::{DistributionKeeper, StakingKeeper},
    BankKeeper, FailingModule, Router, WasmKeeper,
};

pub type BasicRouter<ExecC = Empty, QueryC = Empty> = Router<
    BankKeeper,
    FailingModule<ExecC, QueryC, Empty>,
    WasmKeeper<ExecC, QueryC>,
    StakingKeeper,
    DistributionKeeper,
    IbcKeeper,
    FailingModule<GovMsg, Empty, Empty>,
>;

pub fn mock_router() -> BasicRouter {
    Router {
        wasm: WasmKeeper::new(),
        bank: BankKeeper::new(),
        custom: FailingModule::new(),
        staking: StakingKeeper::new(),
        distribution: DistributionKeeper::new(),
        ibc: IbcKeeper::new(),
        gov: FailingModule::new(),
    }
}
