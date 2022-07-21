use serde::{Deserialize, Serialize};

use crate::endpoint;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Raid {
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaidWing {
    pub id: String,
    pub events: RaidEvent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaidEvent {
    pub id: String,
    pub kind: RaidEventKind,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RaidEventKind {
    Checkpoint,
    Boss,
}

endpoint!(Raid, "/v2/raids", String, get_all);
