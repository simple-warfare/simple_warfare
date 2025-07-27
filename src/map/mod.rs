pub mod navigator;
pub mod physics;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::map::{navigator::SimpleWarfareNavigatorPlugin, physics::SimpleWarfarePhysicsBackend};
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPhysicsPlugin::<SimpleWarfarePhysicsBackend>::default())
            .add_plugins(SimpleWarfareNavigatorPlugin)
            .add_observer(|trigger: Trigger<TiledColliderCreated>, commands: Commands| {});
    }
}
