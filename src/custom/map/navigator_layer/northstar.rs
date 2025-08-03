use bevy::{asset::uuid::Uuid, prelude::*};
use serde::{Deserialize, Serialize};

use crate::custom::map::CustomTile;

#[derive(Debug, Default, Clone, Deserialize, Serialize, Resource)]
pub struct CustomGridLayers {
    pub layer: Vec<CustomGridLayer>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CustomGridLayer {
    pub movement_type: String,
    pub merge_with: Vec<Uuid>,
    pub custom_tile: Vec<CustomTile>,
}
