use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Uint256};

#[cw_serde]
pub enum Cw1155ExecuteMsg {
    SafeTransferFrom { from: Addr, to: Addr, id: Uint256, value: Uint256 },
    SafeBatchTransferFrom { from: Addr, to: Addr, ids: Vec<Uint256>, values: Vec<Uint256> },
    Mint { to: Addr, id: Uint256, value: Uint256 },
    MintBatch { to: Addr, ids: Vec<Uint256>, values: Vec<Uint256> }, 
    Burn { from: Addr, id: Uint256, value: Uint256 },
    BurnBatch { from: Addr, id: Vec<Uint256>, value: Vec<Uint256> },
    SetApprovalForAll { owner: Addr, operator: Addr, approve: bool },
    SendNft { contract: String, id: Uint256, value: Uint256, msg: Binary }
}