use self::errors::unauthorized_admin;
use crate::{Contract, ExecuteCallback, InstantiateCallback, Query};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, ContractInfo, QuerierWrapper, StdResult};

mod errors;

#[cw_serde]
pub enum AdminAuthStatus {
    Active,
    Maintenance,
    Shutdown,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub super_admin: Option<String>,
}

impl InstantiateCallback for InstantiateMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateRegistry { action: RegistryAction },
    UpdateRegistryBulk { actions: Vec<RegistryAction> },
    TransferSuper { new_super: String },
    SelfDestruct {},
    ToggleStatus { new_status: AdminAuthStatus },
}

#[cw_serde]
pub enum RegistryAction {
    RegisterAdmin {
        user: String,
    },
    GrantAccess {
        permissions: Vec<String>,
        user: String,
    },
    RevokeAccess {
        permissions: Vec<String>,
        user: String,
    },
    DeleteAdmin {
        user: String,
    },
}

impl ExecuteCallback for ExecuteMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},
    #[returns(AdminsResponse)]
    GetAdmins {},
    #[returns(PermissionsResponse)]
    GetPermissions { user: String },
    #[returns(ValidateAdminPermissionResponse)]
    ValidateAdminPermission { permission: String, user: String },
}

impl Query for QueryMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub struct ConfigResponse {
    pub super_admin: Addr,
    pub status: AdminAuthStatus,
}

#[cw_serde]
pub struct PermissionsResponse {
    pub permissions: Vec<String>,
}

#[cw_serde]
pub struct AdminsResponse {
    pub admins: Vec<Addr>,
}

#[cw_serde]
pub struct ValidateAdminPermissionResponse {
    pub has_permission: bool,
}

pub fn validate_admin<T: Into<String> + Clone, U: Into<ContractInfo> + Clone>(
    querier: &QuerierWrapper,
    permission: AdminPermissions,
    user: T,
    admin_auth: &U,
) -> StdResult<()> {
    if admin_is_valid(querier, permission.clone(), user.clone(), admin_auth)? {
        Ok(())
    } else {
        Err(unauthorized_admin(&user.into(), &permission.into_string()))
    }
}

pub fn admin_is_valid<T: Into<String>, U: Into<ContractInfo> + Clone>(
    querier: &QuerierWrapper,
    permission: AdminPermissions,
    user: T,
    admin_auth: &U,
) -> StdResult<bool> {
    let admin_resp: StdResult<ValidateAdminPermissionResponse> =
        QueryMsg::ValidateAdminPermission {
            permission: permission.into_string(),
            user: user.into(),
        }
        .query(querier, admin_auth);

    match admin_resp {
        Ok(resp) => Ok(resp.has_permission),
        Err(err) => Err(err),
    }
}

#[derive(Clone)]
pub enum AdminPermissions {
    QueryAuthAdmin,
    ScrtStakingAdmin,
    TreasuryManager,
    TreasuryAdmin,
    StabilityAdmin,
    SkyAdmin,
    LendAdmin,
    OraclesAdmin,
    OraclesPriceBot,
    SilkAdmin,
    ShadeSwapAdmin,
    StakingAdmin,
    DerivativeAdmin,
    Snip20MigrationAdmin,
}

// NOTE: SHADE_{CONTRACT_NAME}_{CONTRACT_ROLE}_{POTENTIAL IDs}

impl AdminPermissions {
    pub fn into_string(self) -> String {
        match self {
            AdminPermissions::QueryAuthAdmin => "SHADE_QUERY_AUTH_ADMIN",
            AdminPermissions::ScrtStakingAdmin => "SHADE_SCRT_STAKING_ADMIN",
            AdminPermissions::TreasuryManager => "SHADE_TREASURY_MANAGER",
            AdminPermissions::TreasuryAdmin => "SHADE_TREASURY_ADMIN",
            AdminPermissions::StabilityAdmin => "SHADE_STABILITY_ADMIN",
            AdminPermissions::SkyAdmin => "SHADE_SKY_ADMIN",
            AdminPermissions::LendAdmin => "SHADE_LEND_ADMIN",
            AdminPermissions::OraclesAdmin => "SHADE_ORACLES_ADMIN",
            AdminPermissions::OraclesPriceBot => "SHADE_ORACLES_PRICE_BOT",
            AdminPermissions::SilkAdmin => "SHADE_SILK_ADMIN",
            AdminPermissions::ShadeSwapAdmin => "SHADE_SWAP_ADMIN",
            AdminPermissions::StakingAdmin => "SHADE_STAKING_ADMIN",
            AdminPermissions::DerivativeAdmin => "SHADE_DERIVATIVE_ADMIN",
            AdminPermissions::Snip20MigrationAdmin => "SNIP20_MIGRATION_ADMIN",
        }
        .to_string()
    }
}
