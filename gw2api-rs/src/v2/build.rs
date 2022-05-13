use crate::Endpoint;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: u64,
}

impl Endpoint for Build {
    type Value = Self;

    const URI: &'static str = "/v2/build";
    const IS_AUTHENTICATED: bool = false;
    const IS_LOCALIZED: bool = false;
}
