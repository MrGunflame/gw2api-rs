use serde::{Deserialize, Serialize};

use crate::endpoint;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    pub id: u64,
    pub name: String,
    pub base_rgb: Vec<u16>,
    pub cloth: ArmorColor,
    pub leather: ArmorColor,
    pub metal: ArmorColor,
    pub fur: Option<ArmorColor>,
    pub item: Option<u64>,
    pub categories: Vec<String>,
}

/// Information about a color applied to an armor.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArmorColor {
    pub brightness: i64,
    pub contrast: f64,
    pub hue: u64,
    pub saturation: f64,
    pub lightness: f64,
    pub rgb: Vec<u16>,
}

endpoint!(Color, "/v2/colors", u64, get_all);
