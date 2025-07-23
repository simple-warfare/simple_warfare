pub mod server;

use bevy::prelude::*;
use bevy_quinnet::client::QuinnetClient;

use crate::{
    bevy_ext::condition::mod_server_has_data, js_engine::JsEngineRequestSender,
    mod_engine::server::ModServer,
};

pub struct ModEnginePlugin;

impl Plugin for ModEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            init_mod_server.run_if(resource_exists::<JsEngineRequestSender>.and(run_once)),
        )
        .add_systems(
            Update,
            handle_mod_server_events
                .run_if(resource_exists::<ModServer>.and(mod_server_has_data())),
        );
    }
}

fn init_mod_server(mut commands: Commands, js_engine_event_sender: Res<JsEngineRequestSender>) {
    commands.insert_resource(ModServer::new(js_engine_event_sender.0.clone()));
}

fn handle_mod_server_events(mut mod_server: ResMut<ModServer>, mut client: ResMut<QuinnetClient>) {
    mod_server
        .client_messages
        .drain(..)
        .for_each(|message| client.connection_mut().send_message(message).unwrap());
}
