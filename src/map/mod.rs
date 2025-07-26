pub mod navigator;
pub mod physics;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use vleue_navigator::VleueNavigatorPlugin;

use crate::map::physics::SimpleWarfarePhysicsBackend;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPhysicsPlugin::<SimpleWarfarePhysicsBackend>::default())
            .add_plugins(VleueNavigatorPlugin);
    }
}
