use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

use crate::custom::unit::turret::JsTurret;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: 42.0,
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                // We can also change color of the overlay
                text_color: Color::WHITE,
                // We can also set the refresh interval for the FPS counter
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
            },
        })
        .add_systems(Update, draw_turret_attack_range);
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
