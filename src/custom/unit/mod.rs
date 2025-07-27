pub mod graphic;
pub mod light2d;
pub mod physics;
pub mod section;
pub mod transform;
pub mod turret;
pub mod unit;
pub mod way_point;

use bevy::prelude::*;

use crate::custom::unit::way_point::WayPointQueue;

#[derive(Event)]
pub struct NewSpawnedUnit(pub Entity);

pub struct CustomUnitPlugin;

impl Plugin for CustomUnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WayPointQueue>();
    }
}
