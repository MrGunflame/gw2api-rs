use serde::{Deserialize, Serialize};

use crate::endpoint;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dungeon {
    pub id: String,
    pub paths: Vec<DungeonPath>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DungeonPath {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: DungeonKind,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DungeonKind {
    Story,
    Explorable,
}

endpoint!(Dungeon, "/v2/dungeons", String, get_all);
