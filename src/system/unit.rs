use std::path::Path;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    custom::unit::{
        NewSpawnedUnit,
        physics::EnablePhysics,
        section::{
            Section,
            core::Core,
            graphic::{Graphic, Graphics},
            movement::Movement,
        },
        turret::JsTurret,
        unit::{Custom, CustomUnit},
        way_point::WayPointQueue,
    },
    js_engine::event::JsEngineResponseEvent,
    spatial::Spatial,
    statistics::*,
};

pub struct UnitSystemPlugin;
impl Plugin for UnitSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewSpawnedUnit>()
            .register_type::<Core>()
            .register_type::<Movement>()
            .register_type::<JsTurret>()
            .register_type::<Section>()
            .register_type::<Graphic>()
            .register_type::<Graphics>()
            .register_type::<Movement>()
            .add_systems(
                Update,
                check_new_unit.run_if(on_event::<JsEngineResponseEvent>),
            );
    }
}
fn check_new_unit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<JsEngineResponseEvent>,
    mut writer: EventWriter<NewSpawnedUnit>,
) -> Result {
    for event in reader.read() {
        if let JsEngineResponseEvent::SpawnedUnit(entity, form, spawned_unit_data) = event {
            let core = &spawned_unit_data.section.core;
            let graphics = &spawned_unit_data.section.graphics;
            let colliders = &spawned_unit_data.section.colliders;
            let point_lights = &spawned_unit_data.section.point_lights;
            let turrets = &spawned_unit_data.section.turrets;
            let turret_entities: Vec<Entity> = turrets
                .data
                .iter()
                .map(|turret| {
                    commands
                        .entity(turret.entity)
                        .insert((
                            Spatial,
                            Custom,
                            turret.clone(),
                            turret.transform.to_transform(),
                            Sprite {
                                image: asset_server.load(
                                    Path::new(form)
                                        .parent()
                                        .unwrap()
                                        .join(turret.image.path.clone()),
                                ),
                                ..Default::default()
                            },
                        ))
                        .id()
                })
                .collect();
            commands
                .entity(*entity)
                .insert((
                    Name::new(core.name.clone()),
                    CustomUnit,
                    Custom,
                    EnablePhysics,
                ))
                .insert((
                    spawned_unit_data.section.clone(),
                    EnablePhysics,
                    Selectable,
                    Spatial,
                    WayPointQueue::default(),
                    RigidBody::Dynamic,
                    MaxLinearSpeed(40.),
                    AngularDamping(0.8),
                    LinearDamping(0.8),
                    ExternalForce::default().with_persistence(false),
                    ComputedMass::new(core.mass),
                    Sprite {
                        image: asset_server.load(
                            Path::new(form)
                                .parent()
                                .unwrap()
                                .join(graphics.data[0].path.clone()),
                        ),
                        ..Default::default()
                    },
                ))
                .add_children(&turret_entities)
                .with_children(|parent| {
                    for collider in colliders.to_avian2d().drain(..) {
                        parent.spawn(collider);
                    }
                    for point_light in point_lights.to_point_light2d().drain(..) {
                        parent.spawn(point_light);
                    }
                });

            writer.write(NewSpawnedUnit(*entity));
        }
    }
    Ok(())
}
