use crate::{Client, Endpoint, Result};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: u64,
}

impl Endpoint for Build {
    type Value = Self;

    const URI: &'static str = "/v2/build";
    const AUTHENTICATED: bool = false;
    const LOCALIZED: bool = false;
}
