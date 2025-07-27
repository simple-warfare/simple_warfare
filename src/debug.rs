use bevy::prelude::*;

use crate::custom::unit::turret::JsTurret;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_turret_attack_range);
    }
}

fn draw_turret_attack_range(
    mut gizmos: Gizmos,
    turret_query: Query<(&JsTurret, &GlobalTransform)>,
) {
    for (turret, global_transform) in turret_query {
        gizmos.circle_2d(
            Isometry2d::from_translation(global_transform.translation().xy()),
            turret.attack_radius,
            Color::WHITE,
        );
    }
}
