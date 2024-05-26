use crate::{BalanceOfBatchResponse, BalanceOfResponse, IsApprovedForAllResponse};
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256};
pub trait CW1155Excute {
    type Err: ToString;

    fn safe_transfer_from(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from: Addr,
        to: Addr,
        id: Uint256,
        value: Uint256,
    ) -> Result<Response, Self::Err>;

    fn safe_batch_transfer_from(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from: Addr,
        to: Addr,
        ids: Vec<Uint256>,
        values: Vec<Uint256>,
    );

    fn mint(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        to: Addr,
        id: Uint256,
        value: Uint256,
    );

    fn mint_batch(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        to: Addr,
        ids: Vec<Uint256>,
        values: Vec<Uint256>,
    );

    fn burn(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from: Addr,
        id: Uint256,
        value: Uint256,
    );

    fn burn_batch(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from: Addr,
        ids: Vec<Uint256>,
        values: Vec<Uint256>,
    );

    fn set_approval_for_all(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        owner: Addr,
        operator: Addr,
        approve: bool,
    );

    fn send_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        contract: String,
        id: Uint256,
        value: Uint256,
        msg: Binary,
    );
}

pub trait CW1155Query {
    fn balance_of(
        &self,
        deps: Deps,
        env: Env,
        account: Addr,
        id: Uint256,
    ) -> StdResult<BalanceOfResponse>;

    fn balance_of_batch(
        &self,
        deps: Deps,
        env: Env,
        accounts: Vec<Addr>,
        ids: Vec<Uint256>,
    ) -> StdResult<BalanceOfBatchResponse>;

    fn is_approve_for_all(
        &self,
        deps: Deps,
        env: Env,
        account: Addr,
        operator: Addr,
    ) -> StdResult<IsApprovedForAllResponse>;
}