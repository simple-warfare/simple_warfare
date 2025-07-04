use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;

use crate::unit::{
    NewSpawnedUnit,
    section::{core::Core, movement::Movement},
};

#[derive(Default, Component)]
pub struct CustomUnitController;
#[derive(Default, Component)]
#[require(CustomUnitController, MovementBundle)]
pub struct EnablePhysics;

#[derive(Default, Component)]
pub struct MovementBundle {
    max_move_speed: Scalar,
    max_turn_speed: Scalar,
    move_acceleration: Scalar,
    move_deceleration: Scalar,
    reverse_percentage: Scalar,
    turn_acceleration: Scalar,
    turn_deceleration: Scalar,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_new_spawned_unit);
    }
}

pub fn check_new_spawned_unit(
    mut event_reader: EventReader<NewSpawnedUnit>,
    mut commands: Commands,
    new_spawned_units: Query<(&Core, &Movement), With<EnablePhysics>>,
) -> Result {
    for NewSpawnedUnit(entity) in event_reader.read() {
        info!("check_new_spawned_unit");
        let (core, movement) = new_spawned_units.get(*entity)?;
        commands.entity(*entity).insert((Mass(core.mass as f32),));
    }
    Ok(())
}
