use bevy::prelude::*;

use crate::{
    custom_unit::unit::CustomUnit,
    js_engine::{
        JsEngineEventRequestSender,
        event::{JsEngineRequestEvent, JsEngineResponseEvent},
        sw::{SwRequestEvent, SwRequestReceiver, TeleportType},
    },
};
pub struct SwPlugin;

impl Plugin for SwPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_sw_event.run_if(resource_exists::<SwRequestReceiver>),
        )
        .add_systems(Update, finish_teleport);
    }
}

fn handle_sw_event(
    sw_event_reader: ResMut<SwRequestReceiver>,
    js_engine_event_sender: Res<JsEngineEventRequestSender>,
) -> Result {
    if let Ok(event) = sw_event_reader
        .0
        .lock()
        .expect("lock js Response receiver error in the system `engine_inited`")
        .try_recv()
    {
        match event {
            SwRequestEvent::Teleport(teleport_type) => match teleport_type {
                TeleportType::Position(js_entity, vec2) => {
                    js_engine_event_sender
                        .0
                        .send(JsEngineRequestEvent::GetEntityToTeleport(js_entity, vec2))?
                    //let mut transform = custom_units.get_mut(entity)?;
                    //transform.translation = Vec3::new(vec2.x, vec2.y, transform.translation.z);
                }
            },
        }
    }

    Ok(())
}

fn finish_teleport(
    mut js_response_reader: EventReader<JsEngineResponseEvent>,
    mut custom_units: Query<&mut Transform, With<CustomUnit>>,
) -> Result {
    for js_response in js_response_reader.read() {
        if let JsEngineResponseEvent::GetedEntityToTeleport(_js_entity, entity, vec2) = *js_response
        {
            let mut transform = custom_units.get_mut(entity)?;
            transform.translation = Vec3::new(vec2.x, vec2.y, transform.translation.z);
        }
    }
    Ok(())
}
