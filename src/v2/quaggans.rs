use serde::{Deserialize, Serialize};

use crate::endpoint;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quaggan {
    pub id: String,
    pub url: String,
}

endpoint!(Quaggan, "/v2/quaggans", String, get_all);
