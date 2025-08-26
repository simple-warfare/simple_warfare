use crate::{
    custom::{
        ui::quick::{QuickDialogData, QuickUi},
        unit::unit::Custom,
    },
    js_engine::{
        event::JsEngineResponseEvent,
        simple_warfare_cli::{
            LookType, SwCliRequestEvent, SwCliRequestReceiver, SwCliResponseEvent,
            SwCliResponseSender, TeleportType,
        },
    },
};
use bevy::prelude::*;
//use bevy_hui::prelude::*;

use super::{io::SwIoPlugin, server::SwServerPlugin};
pub struct SwPlugin;

impl Plugin for SwPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SwIoPlugin)
            .add_plugins(SwServerPlugin)
            .add_systems(
                Update,
                handle_sw_event.run_if(resource_exists::<SwCliRequestReceiver>),
            )
            .add_systems(Update, (finish_teleport, finish_look));
    }
}

fn handle_sw_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sw_cli_request_receiver: ResMut<SwCliRequestReceiver>,
    sw_cli_response_sender: ResMut<SwCliResponseSender>,
) -> Result {
    if let Ok(event) = sw_cli_request_receiver
        .0
        .lock()
        .expect("lock js Response receiver error in the system `engine_inited`")
        .try_recv()
    {
        match event {
            SwCliRequestEvent::RegisterEntity => {
                let entity = commands.spawn_empty().id();
                sw_cli_response_sender
                    .0
                    .send(SwCliResponseEvent::RegisteredEntity(entity))?;
            }
            SwCliRequestEvent::CreateQuickUi(quick_ui) => match quick_ui {
                QuickUi::Dialog(quick_dialog) => match quick_dialog {
                    QuickDialogData::Comfirm(data) => {
                        let node_entity = commands.spawn(data.clone()).id();

                        // commands.entity(node_entity).insert((
                        //     HtmlNode(asset_server.load("mods/std/ui/html/dialog/comfirm.html")),
                        //     TemplateProperties::default()
                        //         .with("node_entity", &serde_json::ser::to_string(&node_entity)?)
                        //         .with("title", &data.title)
                        //         .with("context", &data.context),
                        // ));
                    }
                },
            },
        }
    }
    Ok(())
}

fn finish_teleport(
    mut js_response_reader: EventReader<JsEngineResponseEvent>,
    mut customs: Query<&mut Transform, With<Custom>>,
) -> Result {
    for js_response in js_response_reader.read() {
        if let JsEngineResponseEvent::ToTeleport(telepoty_type) = js_response {
            match telepoty_type {
                TeleportType::Position { this, position } => {
                    let mut transform = customs.get_mut(*this)?;
                    transform.translation =
                        Vec3::new(position.x, position.y, transform.translation.z);
                }
                TeleportType::Entity { this, target } => {
                    let target_transform = *customs.get(*target)?;
                    let mut this_transform = customs.get_mut(*this)?;
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
        if let JsEngineResponseEvent::ToLook(telepoty_type) = js_response {
            match telepoty_type {
                LookType::Position { this, position } => {
                    let mut transform = customs.get_mut(*this)?.0;
                    let target = position.extend(0.);
                    let diff = target - transform.translation;
                    let angle = diff.y.atan2(diff.x);

                    transform.rotation = Quat::from_rotation_z(angle);
                }
                LookType::Entity { this, target } => {
                    let (target_transform, target_global, _) = customs.get(*target)?;
                    let (this_transform, this_global, child_of_option) = customs.get(*this)?;

                    fn calculate_target_rotation(origin: Vec2, target: Vec2) -> Quat {
                        let direction = target - origin;
                        let angle = direction.to_angle();
                        Quat::from_rotation_z(angle)
                    }

                    customs.get_mut(*this)?.0.rotation = match child_of_option {
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
