use crate::endpoint;

use serde::{Deserialize, Serialize};

/// The current build id of the game.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: u64,
}

endpoint!(Build, "/v2/build");
