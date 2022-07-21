use serde::{Deserialize, Serialize};

use crate::endpoint;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Novelty {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub slot: NoveltySlot,
    pub unlock_item: Vec<u64>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NoveltySlot {
    Chair,
    Music,
    HeldItem,
    Miscellaneous,
    Tonic,
}

endpoint!(Novelty, "/v2/novelties", u64, get_all);
