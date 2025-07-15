pub mod server;

use bevy::prelude::*;

use crate::{js_engine::JsEngineRequestSender, mod_engine::server::ModServer};

pub struct ModEnginePlugin;

impl Plugin for ModEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            init_mod_server.run_if(resource_exists::<JsEngineRequestSender>.and(run_once)),
        );
    }
}

fn init_mod_server(
    mut commands: Commands,
    js_engine_event_sender: Res<JsEngineRequestSender>,
) {
    commands.insert_resource(ModServer::new(js_engine_event_sender.0.clone()));
}
