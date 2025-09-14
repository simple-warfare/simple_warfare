use bevy::prelude::*;

use crate::{
    custom::{
        signal::JsSignalStorage,
        unit::{
            section::{graphic::Graphic, movement::Movement},
            turret::JsTurret,
            data::JsUnit,
            way_point::{WayPoint, WayPointQueue},
        },
    },
    js_engine::{JsEngineRequestSender, event::JsEngineRequestEvent, signal::JsSignalType},
    statistics::ServerState,
};

pub struct WayPointSystemPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct HandleWayPointSystem;

impl Plugin for WayPointSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            handle_move_way_point
                .run_if(in_state(ServerState::Starting))
                .in_set(HandleWayPointSystem),
        )
        .add_systems(
            PostUpdate,
            lock_rotation
                .after(TransformSystem::TransformPropagate)
                .run_if(in_state(ServerState::Starting)),
        )
        .add_systems(
            PostUpdate,
            check_active_way_point_changed
                .after(HandleWayPointSystem)
                .run_if(in_state(ServerState::Starting)),
        );
    }
}

fn handle_move_way_point(
    time: Res<Time>,
    way_point_queue: Query<(&mut WayPointQueue, &Transform, &Movement)>,
) {
    use std::f32::consts::*;

    const TWO_PI: f32 = 2.0 * PI;
    const ARRIVAL_THRESHOLD: f32 = 20.0;
    const ANGLE_THRESHOLD_MIN: f32 = FRAC_PI_8 / 10.0;
    const ANGLE_THRESHOLD_MAX: f32 = FRAC_PI_4;
    let delta_time = time.delta_secs();
    for (mut queue, transform, movement) in way_point_queue {
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
                //angular_velocity.0 += angle_diff.signum() * movement.turn_acceleration * delta_time;
                continue;
            } else if angle_diff.abs() > ANGLE_THRESHOLD_MIN {
                //angular_velocity.0 += angle_diff.signum() * movement.turn_deceleration * delta_time;
            } else {
                //angular_velocity.0 = 0.;
            }
            //linear_velocity.0 += direction.normalize() * movement.move_acceleration * delta_time;
        }
    }
}

fn handle_turret_attack_way_point(
    way_point_queue: Query<(&mut WayPointQueue, &Transform, &JsTurret)>,
) {
}

fn lock_rotation(
    graphics_query: Query<(&Graphic, &mut Transform, &ChildOf)>,
    graphics_parent_query: Query<&Transform, Without<Graphic>>,
) {
    for (graphic, mut transform, parent) in graphics_query {
        if let Some(lock_rotation_angle) = graphic.lock_rotation {
            let parent_transform = graphics_parent_query.get(parent.0).unwrap();

            transform.rotation =
                parent_transform.rotation.inverse() * Quat::from_rotation_z(lock_rotation_angle);
        }
    }
}

fn check_active_way_point_changed(
    js_engine_request_sender: Res<JsEngineRequestSender>,
    active_way_point_changed_queue: Query<
        (&WayPointQueue, &JsSignalStorage),
        Changed<WayPointQueue>,
    >,
) -> Result {
    for (quene, js_signal_storage) in active_way_point_changed_queue {
        let Some(new_active_way_point) = quene.data.front() else {
            continue;
        };
        if let Some(new_way_point_signal) = js_signal_storage
            .default_signal_map
            .get(&JsSignalType::ActiveWayPointChanged)
        {
            js_engine_request_sender.0.send(
                JsEngineRequestEvent::active_way_point_changed_signal(
                    new_active_way_point.clone(),
                    new_way_point_signal.entity,
                ),
            )?;
        }
    }

    Ok(())
}
