use std::sync::Arc;

use crate::{
    custom::{
        CustomTypedIdStorage,
        unit::{
            CustomInnerInfo, CustomInnerInfoStorage, NewSpawnedUnit,
            physics::EnablePhysics,
            section::{
                Section,
                core::Core,
                graphic::{Graphic, Graphics},
                movement::Movement,
            },
            turret::JsTurret,
            unit::{CustomTurrrt, CustomUnit},
        },
    },
    js_engine::{
        JsEngineRequestSender,
        event::{JsEngineRequestEvent, JsEngineResponseEvent},
    },
    spatial::Spatial,
    statistics::*,
};
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
            )
            .add_systems(Update, check_new_graphic);
    }
}
fn check_new_unit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<JsEngineResponseEvent>,
    mut writer: EventWriter<NewSpawnedUnit>,
    js_engine_request_sender: Res<JsEngineRequestSender>,
) -> Result {
    for event in reader.read() {
        if let JsEngineResponseEvent::SpawnedUnit { js_unit } = event {
            let ref js_unit_data = js_unit.data;

            let custom_unit_inner_info = Arc::new(CustomInnerInfo::new(&js_unit_data.module_path));
            let custom_inner_info_storage =
                CustomInnerInfoStorage::new(custom_unit_inner_info.clone());
            let custom_typed_id = js_unit_data.custom_typed_id;

            let unit_entity = js_unit_data.entity;

            let core = &js_unit.section.core;
            let graphics = &js_unit.section.graphics;
            let colliders = &js_unit.section.colliders;
            let point_lights = &js_unit.section.point_lights;
            let turrets = &js_unit.section.turrets;

            js_engine_request_sender
                .0
                .send(JsEngineRequestEvent::InsertCustomInnerInfo {
                    entity: unit_entity,
                    custom_inner_info: custom_unit_inner_info.clone(),
                    custom_typed_id,
                })?;

            let turret_entities = turrets
                .data
                .iter()
                .map(|turret| {
                    let turret_image = &turret.image;

                    let image_path = custom_unit_inner_info.get_real_path(&turret_image.path);

                    commands
                        .entity(turret.entity)
                        .insert((
                            Spatial,
                            CustomTurrrt,
                            turret.clone(),
                            Into::<Transform>::into(turret.transform.clone()),
                        ))
                        .id()
                })
                .collect::<Vec<_>>();

            let graphic_entities = graphics
                .data
                .clone()
                .into_iter()
                .map(|graphic| {
                    let graphic_entity = graphic.entity;
                    commands
                        .entity(graphic_entity)
                        .insert((graphic, custom_inner_info_storage.clone()));
                    graphic_entity
                })
                .collect::<Vec<_>>();

            let mut unit_commands = commands.entity(unit_entity);
            unit_commands.insert((
                Name::new(core.name.clone()),
                CustomTypedIdStorage(custom_typed_id),
                CustomUnit,
                js_unit.clone(),
                EnablePhysics,
                Selectable,
                Spatial,
            ));

            unit_commands
                .add_children(&turret_entities)
                .add_children(&graphic_entities);

            writer.write(NewSpawnedUnit(unit_entity));
        }
    }
    Ok(())
}

fn check_new_graphic(
    mut commands: Commands,
    graphic_query: Query<(Entity, &Graphic), Added<Graphic>>,
    asset_server: Res<AssetServer>,
) -> Result {
    for (entity, graphic) in graphic_query {
        let mut entity_commands = commands.entity(entity);
        let Some(real_parent_path) = &graphic.real_parent_path else {
            continue;
        };
    }

    Ok(())
}
