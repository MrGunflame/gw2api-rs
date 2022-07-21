use serde::{Deserialize, Serialize};

use crate::endpoint;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Currency {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub order: u64,
}

endpoint!(Currency, "/v2/currencies", u64, get_all);
