use crate::endpoint;

use serde::{Deserialize, Serialize};

/// The current build id of the game.
///
/// *https://api.guildwars2.com/v2/build*
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: u64,
}

endpoint!(Build, "/v2/build");
