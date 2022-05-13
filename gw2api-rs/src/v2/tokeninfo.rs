use crate::Endpoint;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub id: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub type_: String,
}

impl Endpoint for TokenInfo {
    type Value = Self;

    const URI: &'static str = "/v2/tokeninfo";

    const IS_AUTHENTICATED: bool = true;
    const IS_LOCALIZED: bool = false;
}
