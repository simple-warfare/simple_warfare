use bevy::prelude::*;

use crate::{
    custom_unit::{turret::Turret, unit::CustomUnit},
    js_engine::{
        JsEngineEventRequestSender,
        event::{EntityLookType, EntityTeleportType, JsEngineRequestEvent, JsEngineResponseEvent},
        global::class::entity::JsEntity,
        sw::{SwRequestEvent, SwRequestReceiver, SwResponseEvent, SwResponseSender},
    },
};
pub struct SwPlugin;

impl Plugin for SwPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_sw_event.run_if(resource_exists::<SwRequestReceiver>),
        )
        .add_systems(Update, (finish_teleport, finish_look));
    }
}

fn handle_sw_event(
    mut commands: Commands,
    sw_request_receiver: ResMut<SwRequestReceiver>,
    sw_response_sender: ResMut<SwResponseSender>,
    js_engine_event_sender: Res<JsEngineEventRequestSender>,
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
        }
    }

    Ok(())
}

fn finish_teleport(
    mut js_response_reader: EventReader<JsEngineResponseEvent>,
    mut custom_units: Query<&mut Transform, With<CustomUnit>>,
) -> Result {
    for js_response in js_response_reader.read() {
        if let JsEngineResponseEvent::EntityToTeleport(telepoty_type) = *js_response {
            match telepoty_type {
                EntityTeleportType::Position(entity, vec2) => {
                    let mut transform = custom_units.get_mut(entity)?;
                    transform.translation = Vec3::new(vec2.x, vec2.y, transform.translation.z);
                }
            }
        }
    }
    Ok(())
}

fn finish_look(
    mut js_response_reader: EventReader<JsEngineResponseEvent>,
    mut turrets: Query<&mut Transform, With<Turret>>,
) -> Result {
    for js_response in js_response_reader.read() {
        if let JsEngineResponseEvent::EntityToLook(telepoty_type) = *js_response {
            match telepoty_type {
                EntityLookType::Position(entity, vec2) => {
                    let mut transform = turrets.get_mut(entity)?;
                    let target = vec2.extend(0.);
                    let diff = target - transform.translation;
                    let angle = diff.y.atan2(diff.x);

                    transform.rotation = Quat::from_rotation_z(angle);
                }
            }
        }
    }
    Ok(())
}
