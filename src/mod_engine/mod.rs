pub mod server;

use bevy::prelude::*;

use crate::{js_engine::JsEngineEventRequestSender, mod_engine::server::ModServer};

pub struct ModEnginePlugin;

impl Plugin for ModEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            init_mod_server.run_if(resource_exists::<JsEngineEventRequestSender>.and(run_once)),
        );
    }
}

fn init_mod_server(
    mut commands: Commands,
    js_engine_event_sender: Res<JsEngineEventRequestSender>,
) {
    commands.insert_resource(ModServer::new(js_engine_event_sender.0.clone()));
}
