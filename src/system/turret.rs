use avian2d::prelude::*;
use bevy::prelude::*;
pub struct TurretSystemPlugin;

impl Plugin for TurretSystemPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

fn unit_enter(mut query: Query<(&mut Sprite, &CollidingEntities)>) {}
