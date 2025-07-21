use crate::{
    assets::js_file::JsTomlFile,
    custom::{
        ui::quick::{QuickDialogData, QuickUi},
        unit::unit::Custom,
    },
    js_engine::{
        JsEngineRequestSender,
        event::{EntityLookType, EntityTeleportType, JsEngineRequestEvent, JsEngineResponseEvent},
        global::class::entity::JsEntity,
        sw::{SwRequestEvent, SwRequestReceiver, SwResponseEvent, SwResponseSender},
    },
};
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_hui::prelude::*;
pub struct SwPlugin;

#[derive(Default, Resource)]
pub struct JsReadTomlFiles(pub HashMap<Handle<JsTomlFile>, Vec<Box<oneshot::Sender<String>>>>);

impl Plugin for SwPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<JsReadTomlFiles>()
            .add_systems(
                Update,
                handle_sw_event.run_if(resource_exists::<SwRequestReceiver>),
            )
            .add_systems(Update, check_js_read_file)
            .add_systems(Update, (finish_teleport, finish_look));
    }
}

fn handle_sw_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    js_files: Res<Assets<JsTomlFile>>,
    mut js_read_files: ResMut<JsReadTomlFiles>,
    sw_request_receiver: ResMut<SwRequestReceiver>,
    sw_response_sender: ResMut<SwResponseSender>,
    js_engine_event_sender: Res<JsEngineRequestSender>,
) -> Result {
    if let Ok(event) = sw_request_receiver
        .0
        .lock()
        .expect("lock js Response receiver error in the system `engine_inited`")
        .try_recv()
    {
        match event {
            SwRequestEvent::RegisterEntity => {
                let entity = commands.spawn_empty().id();
                sw_response_sender
                    .0
                    .send(SwResponseEvent::RegisteredEntity(entity))?;
                js_engine_event_sender
                    .0
                    .send(JsEngineRequestEvent::InsertEntity(
                        JsEntity::from_entity(&entity),
                        entity,
                    ))?;
            }
            SwRequestEvent::CreateQuickUi(quick_ui) => match quick_ui {
                QuickUi::Dialog(quick_dialog) => match quick_dialog {
                    QuickDialogData::Comfirm(data) => {
                        let node_entity = commands.spawn(data.clone()).id();

                        commands.entity(node_entity).insert((
                            HtmlNode(asset_server.load("mods/std/ui/html/dialog/comfirm.html")),
                            TemplateProperties::default()
                                .with("node_entity", &serde_json::ser::to_string(&node_entity)?)
                                .with("title", &data.title)
                                .with("context", &data.context),
                        ));
                    }
                },
            },
            SwRequestEvent::ReadFile(sender, file_path) => {
                let file_handle = asset_server.load(file_path);
                if asset_server.is_loaded(file_handle.id()) {
                    sender.send(js_files.get(file_handle.id()).unwrap().data.clone())?;
                } else {
                    if let Some(file_senders) = js_read_files.0.get_mut(&file_handle) {
                        file_senders.push(sender);
                    } else {
                        js_read_files.0.insert(file_handle, vec![sender]);
                    }
                }

                //
            }
        }
    }
    Ok(())
}

fn check_js_read_file(
    asset_server: Res<AssetServer>,
    mut evnets: EventReader<AssetEvent<JsTomlFile>>,
    js_files: Res<Assets<JsTomlFile>>,
    mut js_read_files: ResMut<JsReadTomlFiles>,
) -> Result {
    for event in evnets.read() {
        if let AssetEvent::LoadedWithDependencies { id } = *event {
            if let Some(mut senders) = js_read_files
                .0
                .remove(&asset_server.get_id_handle(id).unwrap())
            {
                senders.drain(..).for_each(|sender| {
                    sender.send(js_files.get(id).unwrap().data.clone()).unwrap();
                });
            }
        }
    }
    Ok(())
}
fn finish_teleport(
    mut js_response_reader: EventReader<JsEngineResponseEvent>,
    mut customs: Query<&mut Transform, With<Custom>>,
) -> Result {
    for js_response in js_response_reader.read() {
        if let JsEngineResponseEvent::EntityToTeleport(telepoty_type) = *js_response {
            match telepoty_type {
                EntityTeleportType::Position(entity, vec2) => {
                    let mut transform = customs.get_mut(entity)?;
                    transform.translation = Vec3::new(vec2.x, vec2.y, transform.translation.z);
                }
                EntityTeleportType::Entity(this_entity, target_entity) => {
                    let target_transform = customs.get(target_entity)?.clone();
                    let mut this_transform = customs.get_mut(this_entity)?;
                    *this_transform = target_transform;
                }
            }
        }
    }
    Ok(())
}

fn finish_look(
    mut js_response_reader: EventReader<JsEngineResponseEvent>,
    mut customs: Query<(&mut Transform, &GlobalTransform, Option<&ChildOf>), With<Custom>>,
) -> Result {
    for js_response in js_response_reader.read() {
        if let JsEngineResponseEvent::EntityToLook(telepoty_type) = *js_response {
            match telepoty_type {
                EntityLookType::Position(entity, vec2) => {
                    let mut transform = customs.get_mut(entity)?.0;
                    let target = vec2.extend(0.);
                    let diff = target - transform.translation;
                    let angle = diff.y.atan2(diff.x);

                    transform.rotation = Quat::from_rotation_z(angle);
                }
                EntityLookType::Entity(this_entity, target_entity) => {
                    let (target_transform, target_global, _) = customs.get(target_entity)?;
                    let (this_transform, this_global, child_of_option) =
                        customs.get(this_entity)?;

                    fn calculate_target_rotation(origin: Vec2, target: Vec2) -> Quat {
                        let direction = target - origin;
                        let angle = direction.to_angle();
                        Quat::from_rotation_z(angle)
                    }

                    customs.get_mut(this_entity)?.0.rotation = match child_of_option {
                        Some(child_of) => {
                            customs.get(child_of.0)?.1.rotation().inverse()
                                * calculate_target_rotation(
                                    this_global.translation().truncate(),
                                    target_global.translation().truncate(),
                                )
                        }

                        None => calculate_target_rotation(
                            this_transform.translation.truncate(),
                            target_transform.translation.truncate(),
                        ),
                    }
                }
            }
        }
    }
    Ok(())
}
