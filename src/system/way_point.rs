use avian2d::prelude::{AngularVelocity, LinearVelocity};
use bevy::{pbr::graph, prelude::*, reflect::List};
use bevy_inspector_egui::egui::{debug_text::print, emath::easing::linear};

use crate::{
    custom::{
        signal::JsSignalStorage,
        unit::{
            section::{graphic::Graphic, movement::Movement},
            turret::JsTurret,
            unit::JsUnit,
            way_point::{WayPoint, WayPointQueue},
        },
    },
    js_engine::{event::JsEngineRequestEvent, signal::JsSignalType, JsEngineRequestSender},
    scenes::SceneState, system::physics::*,
};

pub struct WayPointSystemPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct HandleWayPointSystem;

impl Plugin for WayPointSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPreUpdate,
            handle_move_way_point
                .run_if(in_state(SceneState::GameScene))
                .in_set(HandleWayPointSystem),
        )
        .add_systems(
            PostUpdate,
            lock_rotation
                .after(TransformSystem::TransformPropagate)
                .run_if(in_state(SceneState::GameScene)),
        )
        .add_systems(
            FixedPreUpdate,
            check_active_way_point_changed
                .after(HandleWayPointSystem)
                .run_if(in_state(SceneState::GameScene)),
        );
    }
}
fn handle_move_way_point(
    time: Res<Time>,
    way_point_queue: Query<(
        &mut WayPointQueue,
        &mut Transform,
        &Movement,
        &mut AngularSpeed,
        &mut AngularOffset,
        &mut LinearSpeed,
        &mut LinearOffset,
    )>,
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
        //mut external_force,
        mut angular_speed,
        mut angular_offset,
        mut linear_speed,
        mut linear_offset,
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
                linear_speed.0-=movement.move_deceleration * delta_time;
                if linear_speed.0 < 0.0 {
                    linear_speed.0 = 0.0;
                } 
                angular_speed.0 += movement.turn_acceleration * delta_time;
            } else if angle_diff.abs() > ANGLE_THRESHOLD_MIN {
                angular_speed.0 += movement.turn_acceleration * delta_time;
                linear_speed.0 += movement.move_acceleration * delta_time;
                if angle_diff.abs()<=angular_speed.0 * delta_time{
                    angular_speed.0=angle_diff * delta_time;
                }
            } else {
                angular_speed.0 = 0.;
                linear_speed.0 += movement.move_acceleration * delta_time;
                //transform.rotate_z(angle_diff);
            }
            if angular_speed.0 > movement.max_turn_speed {
                angular_speed.0 = movement.max_turn_speed;
            }
            linear_offset.0 = Vec2::new(current_angle.cos(),current_angle.sin()) * linear_speed.0 * delta_time;//按照自身方向移动
            angular_offset.0 += angle_diff.signum() * angular_speed.0 * delta_time;
            if linear_speed.0 > movement.max_move_speed {
                linear_speed.0 = movement.max_move_speed;
            }
        }

        else if queue.data.is_empty() {//无路径点时停止移动
            angular_speed.0 = 0.0;
            
            linear_speed.0 -=movement.move_deceleration * delta_time;
            if linear_speed.0<0.0{
                linear_speed.0 = 0.0;
            }
            let current_angle = transform.rotation.to_euler(EulerRot::ZYX).0 + FRAC_PI_2;
            if linear_speed.0!=0.0{
                linear_offset.0 = Vec2::new(current_angle.cos(),current_angle.sin()) * linear_speed.0 * delta_time;//按照自身方向移动
            }
            else{
                linear_offset.0=Vec2::ZERO;
            }
        }
    }
}

fn handle_turret_attack_way_point(
    way_point_queue: Query<(
        &mut WayPointQueue,
        &Transform,
        &JsTurret,
        &mut AngularVelocity,
        &mut LinearVelocity,
    )>,
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
