use crate::{
    custom::{
        CustomTypedIdStorage,
        unit::{
            CustomInnerInfo, NewSpawnedUnit,
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
    },
    js_engine::{
        JsEngineRequestSender,
        event::{JsEngineRequestEvent, JsEngineResponseEvent},
    },
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
    js_engine_request_sender: Res<JsEngineRequestSender>,
) -> Result {
    for event in reader.read() {
        if let JsEngineResponseEvent::SpawnedUnit { data } = event {
            unit_mapping.add_entity(data.unit_id, data.entity);

            let custom_unit_inner_info = CustomInnerInfo::new(&data.module_path);
            let custom_typed_id = data.custom_typed_id;

            let unit_entity = data.entity;
            let core = &data.section.core;
            let graphics = &data.section.graphics;
            let colliders = &data.section.colliders;
            let point_lights = &data.section.point_lights;
            let turrets = &data.section.turrets;

            js_engine_request_sender
                .0
                .send(JsEngineRequestEvent::InsertCustomInnerInfo {
                    entity: unit_entity,
                    custom_inner_info: custom_unit_inner_info.clone(),
                    custom_typed_id,
                })?;

            let turret_entities: Vec<Entity> = turrets
                .data
                .iter()
                .map(|turret| {
                    let turret_image = &turret.image;

                    let image_path = custom_unit_inner_info.get_real_path(&turret_image.path);

                    let anchor = turret.image.anchor();
                    let sprite = Sprite {
                        image: asset_server.load(image_path),
                        anchor,
                        ..Default::default()
                    };
                    commands
                        .entity(turret.entity)
                        .insert((
                            Spatial,
                            Custom,
                            CustomUnit,
                            sprite,
                            turret.clone(),
                            turret.transform.to_transform(),
                        ))
                        .id()
                })
                .collect();

            commands
                .entity(unit_entity)
                .insert((
                    Name::new(core.name.clone()),
                    CustomTypedIdStorage(custom_typed_id),
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

            writer.write(NewSpawnedUnit(unit_entity));
        }
    }
    Ok(())
}
