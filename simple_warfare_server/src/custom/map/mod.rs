use bevy_northstar::prelude::*;
use serde::{Deserialize, Serialize};

pub mod navigator_layer;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomTile {
    pub user_type: String,
    pub nav: Nav,
}
