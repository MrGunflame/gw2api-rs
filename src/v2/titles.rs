use serde::{Deserialize, Serialize};

use crate::endpoint;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Title {
    pub id: u64,
    pub name: String,
    pub achievements: Option<Vec<u64>>,
    pub ap_required: Option<u64>,
}

endpoint!(Title, "/v2/titles", u64, get_all);
