pub mod server;

use bevy::prelude::*;

use crate::js_engine::{
    JsEngineRequestSender, JsEngineResponseReciver, event::JsEngineResponseEvent,
};

use self::server::ModServer;

pub struct ModEnginePlugin;

impl Plugin for ModEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            init_mod_server.run_if(resource_exists::<JsEngineRequestSender>.and(run_once)),
        )
        .add_systems(
            Update,
            loaded_custom_units_event.run_if(resource_exists::<JsEngineResponseReciver>),
        );
    }
}

fn init_mod_server(mut commands: Commands, js_engine_event_sender: Res<JsEngineRequestSender>) {
    commands.insert_resource(ModServer::new(js_engine_event_sender.0.clone()));
}

fn loaded_custom_units_event(
    mut reader: EventReader<JsEngineResponseEvent>,
    mut mod_server: ResMut<ModServer>,
) {
    for event in reader.read() {
        if let JsEngineResponseEvent::LoadedCustomUnits { loaded_number } = event {
            mod_server.loaded_custom_unit_number += loaded_number
        }
    }
}
