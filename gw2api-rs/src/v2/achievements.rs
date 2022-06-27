use crate::endpoint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub id: u64,
    pub icon: Option<String>,
    pub name: String,
    pub description: String,
    pub requirement: String,
    pub locked_text: String,
    #[serde(rename = "type")]
    pub kind: AchievementKind,
    pub flags: Vec<String>,
    pub tiers: Vec<AchievementTier>,
    #[serde(default)]
    pub prerequisites: Vec<u64>,
    #[serde(default)]
    pub rewards: Vec<AchievementReward>,
    #[serde(default)]
    pub bits: Vec<AchievementBit>,
    pub point_cap: Option<u64>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementKind {
    Default,
    ItemSet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementTier {
    pub count: u64,
    pub points: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AchievementReward {
    Coins { count: u64 },
    Item { id: u64, count: u64 },
    Mastery { id: u64, region: String },
    Title { id: u64 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AchievementBit {
    Text { text: String },
    Item { id: u64 },
    Minipet { id: u64 },
    Skin { id: u64 },
}

endpoint!(Achievement, "/v2/achievements", u64);
