use bevy::{platform::collections::HashSet, prelude::*};
use bevy_spatial::SpatialAccess;

use crate::{
    custom::unit::{turret::JsTurret, unit::CustomUnit},
    js_engine::{
        JsEngineEventRequestSender, event::JsEngineRequestEvent, global::class::entity::JsEntity,
    },
    spatial::{Spatial, SpatialTree},
};
pub struct TurretSystemPlugin;

impl Plugin for TurretSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            unit_enter.run_if(resource_exists::<JsEngineEventRequestSender>),
        );
    }
}

fn unit_enter(
    treeaccess: Res<SpatialTree>,
    unit_query: Query<Entity, (With<Spatial>, With<CustomUnit>)>,
    turret_query: Query<(&ChildOf, &mut JsTurret, &GlobalTransform), With<Spatial>>,
    js_engine_request_sender: Res<JsEngineEventRequestSender>,
) -> Result {
    for (child_of, mut turret, turret_pos) in turret_query {
        let units_in_range: Vec<Entity> = treeaccess
            .within_distance(turret_pos.translation().xy(), turret.attack_radius)
            .iter()
            .filter_map(|(_, entity)| Some((*entity)?))
            .collect();

        let mut new_units_in_range = Vec::new();
        let mut current_units_set = HashSet::new();

        for entity in &units_in_range {
            if let Ok(_) = unit_query.get(*entity) {
                let js_entity = JsEntity::from_entity(entity);
                current_units_set.insert(js_entity.clone());

                if !turret.units_in_range.contains(&js_entity) && child_of.0 != *entity {
                    turret.units_in_range.push(js_entity.clone());
                    new_units_in_range.push(js_entity);
                }
            }
        }

        let mut exited_units = Vec::new();
        turret.units_in_range.retain(|js_entity| {
            if current_units_set.contains(js_entity) {
                true
            } else {
                exited_units.push(js_entity.clone());
                false
            }
        });

        if !new_units_in_range.is_empty() {
            js_engine_request_sender
                .0
                .send(JsEngineRequestEvent::OnUnitEnterSignal(
                    new_units_in_range,
                    turret.on_unit_enter_signal_entity,
                ))?;
        }

        if !exited_units.is_empty() {
            js_engine_request_sender
                .0
                .send(JsEngineRequestEvent::OnUnitExitSignal(
                    exited_units,
                    turret.on_unit_exit_signal_entity,
                ))?;
        }
    }
    Ok(())
}
