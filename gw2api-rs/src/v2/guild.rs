use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub emblem: GuildEmblem,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildEmblem {
    pub background: GuildEmblemSection,
    pub foreground: GuildEmblemSection,
    pub flags: Vec<GuildEmblemFlag>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildEmblemSection {
    pub id: u64,
    pub colors: Vec<u64>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum GuildEmblemFlag {
    FlipBackgroundHorizontal,
    FlipBackgroundVertical,
}
