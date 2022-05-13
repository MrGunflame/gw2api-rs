use crate::{Endpoint, ListEndpoint};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// A WvW ability
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ability {
    pub id: u64,
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

impl Endpoint for Ability {
    type Value = Vec<u64>;

    const URI: &'static str = "/v2/wvw/abilities";

    const IS_AUTHENTICATED: bool = false;
    const IS_LOCALIZED: bool = true;
}

impl ListEndpoint for Ability {
    type Id = u64;

    type SingleValue = Self;
    type MultiValue = Vec<Self>;
}

/// Details about a WvW match
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Match {
    /// Id of the match. NA matches have an `id` starting with `1`, EU matches have an `id`
    /// starting with `2`. The second number indicates the tier of match. Matches are sorted after
    /// their tier, but **not** NA/EU matches.
    pub id: String,
    /// The starting time of the match.
    pub start_time: DateTime<Utc>,
    /// The ending time of the match.
    pub end_time: DateTime<Utc>,
    /// The total (sum) scores of all sides. The keys always include `red`, `green` and `blue`.
    pub scores: HashMap<String, u64>,
    /// The hosting worlds of all sides.
    pub worlds: HashMap<String, u64>,
    /// All worlds of all sides.
    pub all_worlds: HashMap<String, Vec<u64>>,
    /// The total deaths of all sides.
    pub deaths: HashMap<String, u64>,
    /// The total kills of all sides.
    pub kills: HashMap<String, u64>,
    /// The total victory points of all sides.
    pub victory_points: HashMap<String, u64>,
    /// Detailed skirmish information (2 hours)
    pub skirmishes: Vec<Skirmish>,
    /// Map information
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
    /// Only for camps, towers and keeps (and sm)
    pub yaks_delivered: Option<u64>,
    pub guild_upgrades: Option<Vec<u64>>,
}

impl Endpoint for Match {
    type Value = Vec<String>;

    const URI: &'static str = "/v2/wvw/matches";

    const IS_AUTHENTICATED: bool = false;
    const IS_LOCALIZED: bool = false;
}

impl ListEndpoint for Match {
    type Id = String;

    type SingleValue = Self;
    type MultiValue = Vec<Self>;
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

    const IS_AUTHENTICATED: bool = false;
    const IS_LOCALIZED: bool = false;
}

impl ListEndpoint for Rank {
    type Id = u64;

    type SingleValue = Self;
    type MultiValue = Vec<Self>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Upgrades {
    pub id: u64,
    pub tiers: Vec<UpgradeTier>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeTier {
    pub name: String,
    pub yaks_required: u64,
    pub upgrades: Vec<Upgrade>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Upgrade {
    pub name: String,
    pub description: String,
    pub icon: String,
}

impl Endpoint for Upgrades {
    type Value = Vec<u64>;

    const URI: &'static str = "/v2/wvw/upgrades";

    const IS_AUTHENTICATED: bool = false;
    const IS_LOCALIZED: bool = true;
}

impl ListEndpoint for Upgrades {
    type Id = u64;

    type SingleValue = Self;
    type MultiValue = Vec<Self>;
}
