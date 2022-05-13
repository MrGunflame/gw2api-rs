use crate::{Client, Result};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub id: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub type_: String,
}
