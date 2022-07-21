use serde::{Deserialize, Serialize};

use crate::endpoint;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub icon: String,
}

endpoint!(File, "/v2/files", String, get_all);
