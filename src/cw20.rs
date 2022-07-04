use std::{convert::TryFrom, error::Error};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdError, StdResult, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;

use crate::{Asset, AssetInfo, Mint};

pub struct Cw20 {
    pub address: Addr,
    pub amount: Uint128,
}

impl TryFrom<Asset> for Cw20 {
    type Error = StdError;

    fn try_from(asset: Asset) -> StdResult<Self> {
        match asset.info {
            AssetInfo::Cw20(address) => Ok(Cw20 {
                address,
                amount: asset.amount,
            }),
            AssetInfo::Native(_) => {
                Err(StdError::generic_err("Cannot convert native asset to Cw20."))
            }
        }
    }
}

impl Mint for Cw20 {
    fn mint_msg<A: Into<String>, B: Into<String>>(
        &self,
        _sender: A,
        recipient: B,
    ) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self.address.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Mint {
                recipient: recipient.into(),
                amount: self.amount,
            })?,
            funds: vec![],
        }))
    }
}
