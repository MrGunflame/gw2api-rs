use crate::{Client, Endpoint, Result};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Abilities {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub ranks: Vec<AbilityRank>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityRank {
    pub cost: u64,
    pub effect: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Match {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub scores: HashMap<String, u64>,
    pub worlds: HashMap<String, u64>,
    pub all_worlds: HashMap<String, Vec<u64>>,
    pub deaths: HashMap<String, u64>,
    pub kills: HashMap<String, u64>,
    pub victory_points: HashMap<String, u64>,
    pub skrimishes: Vec<Skirmish>,
    pub maps: Vec<Map>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skirmish {
    pub id: u64,
    pub scores: HashMap<String, u64>,
    pub map_scores: Vec<MapScore>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapScore {
    pub r#type: String,
    pub scores: HashMap<String, u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub id: u64,
    pub r#type: String,
    pub scores: HashMap<String, u64>,
    pub bonuses: Vec<Bonus>,
    pub objectives: Vec<Objective>,
    pub deaths: HashMap<String, u64>,
    pub kills: HashMap<String, u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bonus {
    pub r#type: String,
    pub owner: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Objective {
    pub id: String,
    pub r#type: String,
    pub owner: String,
    pub last_flipped: DateTime<Utc>,
    pub claimed_by: Option<String>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub points_tick: u64,
    pub points_capture: u64,
    pub yaks_delivered: u64,
    pub guild_upgrades: Vec<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rank {
    pub id: u64,
    pub title: String,
    pub min_rank: u64,
}

impl Endpoint for Rank {
    type Value = Vec<u64>;

    const URI: &'static str = "/v2/wvw/ranks";
    const AUTHENTICATED: bool = false;
    const LOCALIZED: bool = false;
}
