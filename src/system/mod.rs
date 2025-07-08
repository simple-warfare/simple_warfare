mod input;
mod physics;
mod turret;
mod unit;
mod way_point;
use bevy::prelude::*;

use crate::system::{
    input::InputSystemPlugin, physics::PhysicsSystemPlugin, turret::TurretSystemPlugin,
    unit::UnitSystemPlugin, way_point::WayPointSystemPlugin,
};

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputSystemPlugin,
            UnitSystemPlugin,
            PhysicsSystemPlugin,
            WayPointSystemPlugin,
            TurretSystemPlugin,
        ));
    }
}
