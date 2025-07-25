use std::path::Path;

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
    net::shared::UnitMapping,
    spatial::Spatial,
    statistics::*,
};
use avian2d::prelude::*;
use bevy::prelude::*;

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
                FixedUpdate,
                check_new_unit.run_if(on_event::<JsEngineResponseEvent>),
            );
    }
}
fn check_new_unit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<JsEngineResponseEvent>,
    mut writer: EventWriter<NewSpawnedUnit>,
    mut unit_mapping: ResMut<UnitMapping>,
) -> Result {
    for event in reader.read() {
        if let JsEngineResponseEvent::SpawnedUnit {
            unit_id,
            entity,
            module_path,
            data,
        } = event
        {
            unit_mapping.add_entity(*unit_id, *entity);
            let core = &data.section.core;
            let graphics = &data.section.graphics;
            let colliders = &data.section.colliders;
            let point_lights = &data.section.point_lights;
            let turrets = &data.section.turrets;
            let turret_entities: Vec<Entity> = turrets
                .data
                .iter()
                .map(|turret| {
                    let image_path = Path::new(module_path)
                        .parent()
                        .unwrap()
                        .join(turret.image.path.clone());

                    let anchor = turret.image.anchor();
                    let sprite = Sprite {
                        image: asset_server.load(image_path),
                        anchor: anchor,
                        ..Default::default()
                    };
                    commands
                        .entity(turret.entity)
                        .insert((
                            Spatial,
                            Custom,
                            sprite,
                            turret.clone(),
                            turret.transform.to_transform(),
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
                    data.section.clone(),
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
                            Path::new(module_path)
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
