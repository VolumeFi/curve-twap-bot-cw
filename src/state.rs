use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::Metadata;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub job_id: String,
    pub owner: Addr,
    pub metadata: Metadata,
}

pub const STATE: Item<State> = Item::new("state");
