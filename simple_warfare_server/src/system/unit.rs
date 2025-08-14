use std::{path::Path, sync::Arc};

use crate::{
    bevy_ext::error::CommonBevyError,
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
    net::shared::UnitMapping,
    shared::SharedCutomHandleMapping,
    spatial::Spatial,
    statistics::*,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_northstar::prelude::*;
use bevy_trickfilm::prelude::*;

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
    mut unit_mapping: ResMut<UnitMapping>,
    js_engine_request_sender: Res<JsEngineRequestSender>,
) -> Result {
    for event in reader.read() {
        if let JsEngineResponseEvent::SpawnedUnit { js_unit } = event {
            let ref js_unit_data = js_unit.data;
            unit_mapping.add_entity(js_unit_data.unit_id, js_unit_data.entity);

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
                            CustomTurrrt,
                            sprite,
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
                RigidBody::Dynamic,
                MaxLinearSpeed(40.),
                AngularDamping(0.8),
                LinearDamping(0.8),
                ExternalForce::default().with_persistence(false),
                ComputedMass::new(core.mass),
            ));

            unit_commands
                .add_children(&turret_entities)
                .add_children(&graphic_entities)
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

fn check_new_graphic(
    mut commands: Commands,
    graphic_query: Query<(Entity, &Graphic), Added<Graphic>>,
    asset_server: Res<AssetServer>,
    mut shared_cutom_handle_mapping: ResMut<SharedCutomHandleMapping>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> Result {
    for (entity, graphic) in graphic_query {
        let mut entity_commands = commands.entity(entity);
        let Some(real_parent_path) = &graphic.real_parent_path else {
            continue;
        };

        let image_path = Path::new(real_parent_path).join(&graphic.path);

        let sprite = if let (Some(frame_width), Some(frame_height)) =
            (graphic.frame_width, graphic.frame_height)
        {
            let layout = TextureAtlasLayout::from_grid(
                UVec2::new(frame_width, frame_height),
                graphic.width / frame_width,
                graphic.height / frame_height,
                None,
                None,
            );
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            Sprite {
                image: asset_server.load(image_path),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                }),
                ..Default::default()
            }
        } else {
            Sprite {
                image: asset_server.load(image_path),
                ..Default::default()
            }
        };

        entity_commands.insert(sprite);

        if graphic.trick_film_player.is_some() {
            entity_commands.insert(AnimationPlayer2D::default());
        }
    }

    Ok(())
}
