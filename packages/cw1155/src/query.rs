use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint256};

#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw1155QueryMsg {

    #[returns(BalanceOfResponse)]
    BalanceOf { account: Addr, id: Uint256 },

    #[returns(BalanceOfBatchResponse)]
    BalanceOfBatch { accounts: Vec<Addr>, ids: Vec<Uint256> },

    #[returns(IsApprovedForAllResponse)]
    IsApprovedForAll { account: Addr, operator: Addr }
}

#[cw_serde]
pub struct BalanceOfResponse {
    pub balance: Uint256,
}

#[cw_serde]
pub struct BalanceOfBatchResponse {
    pub balances: Vec<Uint256>
}

#[cw_serde]
pub struct IsApprovedForAllResponse {
    pub approved: bool,
}