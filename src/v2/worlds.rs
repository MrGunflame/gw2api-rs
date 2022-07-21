use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::endpoint;

/// A game world.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
    pub id: u64,
    pub name: String,
    pub population: Population,
}

/// The population of a [`World`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Population {
    Full,
    VeryHigh,
    High,
    Medium,
}

impl Population {
    #[inline]
    fn as_u8(&self) -> u8 {
        match self {
            Self::Full => 3,
            Self::VeryHigh => 2,
            Self::High => 1,
            Self::Medium => 0,
        }
    }
}

impl PartialOrd for Population {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_u8().partial_cmp(&other.as_u8())
    }
}

endpoint!(World, "/v2/worlds", u64, get_all);
