use cosmwasm_std::{Binary, Uint128};
use cw0::Expiration;
use cw20::Logo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub cw20_token_address: String,
    pub admin_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateLunaUserList {
        count: usize,
        user_name_list: Vec<String>,
        luna_airdrop_qualified_list: Vec<bool>, 
        luna_airdrop_reward_amount_list: Vec<Uint128>,
    },
    SetContractLockStatus { 
        lock_status: Uint128,
    },
    CreateLunaUser { 
        user_name: String,
        luna_airdrop_qualified: bool, 
        luna_airdrop_reward_amount: Uint128,
    },
    UpdateLunaUser { 
        user_name: String,
        luna_airdrop_qualified: bool, 
        luna_airdrop_reward_amount: Uint128,
    },
    CreateActivity { 
        activity_name: String,
        eligible_activity_reward_amount: Uint128,
    },
    SetActivityRewardAmount {
        activity_name: String,
        eligible_activity_reward_amount: Uint128,
    },
    UpdateUserActivity {
        user_name: String,
        activity_name: String,
        activity_qualified: bool,
    },
    ClaimUserRewards {
        user_name: String,
    },
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    BurnFrom {
        owner: String,
        amount: Uint128,
    },
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    /// Return type: AllowanceResponse.
    Allowance {
        owner: String,
        spender: String,
    },
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this owner has approved. Supports pagination.
    /// Return type: AllAllowancesResponse.
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Returns the current state of activity information for the given address.
    /// Return type: UserActivityDetails.
    UserActivityDetails {
        user_name: String,
    },
}
