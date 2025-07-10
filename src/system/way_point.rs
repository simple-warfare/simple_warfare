use avian2d::
    prelude::{AngularVelocity, ExternalForce, LinearVelocity}
;
use bevy::prelude::*;

use crate::{
    custom::unit::{
        section::movement::Movement,
        way_point::{WayPoint, WayPointQueue},
    },
    scenes::SceneState,
};

pub struct WayPointSystemPlugin;

impl Plugin for WayPointSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            handle_move_way_point.run_if(in_state(SceneState::GameScene)),
        );
    }
}

fn handle_move_way_point(
    time: Res<Time>,
    way_point_queue: Query<
        (
            &mut WayPointQueue,
            &mut Transform,
            &Movement,
            &mut ExternalForce,
            &mut AngularVelocity,
            &mut LinearVelocity,
        ),
        With<Movement>,
    >,
) {
    use std::f32::consts::*;

    const TWO_PI: f32 = 2.0 * PI;
    const ARRIVAL_THRESHOLD: f32 = 20.0;
    const ANGLE_THRESHOLD_MIN: f32 = FRAC_PI_8 / 10.0;
    const ANGLE_THRESHOLD_MAX: f32 = FRAC_PI_4;
    let delta_time = time.delta_secs();
    for (
        mut queue,
        mut transform,
        movement,
        mut external_force,
        mut angular_velocity,
        mut linear_velocity,
    ) in way_point_queue
    {
        if let Some(WayPoint::Move(target)) = queue.data.front() {
            let direction = target - transform.translation.xy();
            let distance = direction.length();
            if distance <= ARRIVAL_THRESHOLD {
                queue.data.pop_front();
                continue;
            }

            let target_angle = direction.to_angle();
            let current_angle = transform.rotation.to_euler(EulerRot::ZYX).0 + FRAC_PI_2;

            let mut angle_diff = target_angle - current_angle;
            angle_diff = (angle_diff + PI).rem_euclid(TWO_PI) - PI;

            if angle_diff.abs() > ANGLE_THRESHOLD_MAX {
                angular_velocity.0 += angle_diff.signum() * movement.move_acceleration * delta_time;
                continue;
            } else if angle_diff.abs() > ANGLE_THRESHOLD_MIN {
                angular_velocity.0 += angle_diff.signum() * movement.move_acceleration * delta_time;
            } else {
                angular_velocity.0 = 0.;
            }
            external_force.apply_force(direction.normalize().xy() * 80000.);
        }
    }
}
