pub mod navigator;
pub mod northstar;
pub mod physics;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::map::{
    navigator::SimpleWarfareNavigatorPlugin, northstar::SimpleWarfareNorthStarPlugin,
    physics::TiledPhysicsParryBackend,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPhysicsPlugin::<TiledPhysicsParryBackend>::default())
            .add_plugins(SimpleWarfareNavigatorPlugin);
    }
}
