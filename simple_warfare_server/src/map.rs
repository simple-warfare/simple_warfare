pub mod navigator;
pub mod northstar;
pub mod physics;

use bevy::prelude::*;

use crate::map::{
    northstar::SimpleWarfareNorthStarPlugin,
    physics::TiledPhysicsParryBackend,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(TiledPlugin::default())
        //     .add_plugins(TiledPhysicsPlugin::<TiledPhysicsParryBackend>::default())
        //     .add_plugins(SimpleWarfareNorthStarPlugin);
    }
}
