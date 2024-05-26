use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Addr, Binary, CosmosMsg, StdResult, Uint256, WasmMsg};
use schemars::JsonSchema;

#[cw_serde]
pub struct Cw1155ReceiveMsg {
    pub id: Uint256,
    pub sender: Addr,
    pub value: Uint256,
    pub msg: Binary,
}

impl Cw1155ReceiveMsg {
    pub fn _into_cosmos_msg<T, C>(self, contract_address: T) -> StdResult<CosmosMsg<C>>
    where
        T: Into<String>,
        C: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        let msg = to_json_binary(&ReceiverExecuteMsg::ReceiveCW1155Nft(self))?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_address.into(),
            msg,
            funds: vec![],
        };

        Ok(execute.into())
    }
}

#[cw_serde]
pub enum ReceiverExecuteMsg {
    ReceiveCW1155Nft(Cw1155ReceiveMsg)
}