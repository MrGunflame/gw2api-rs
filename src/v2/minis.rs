use serde::{Deserialize, Serialize};

use crate::endpoint;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mini {
    pub id: u64,
    pub name: String,
    pub unlock: Option<String>,
    pub icon: String,
    pub order: u64,
    pub item_id: u64,
}

endpoint!(Mini, "/v2/minis", u64, get_all);
