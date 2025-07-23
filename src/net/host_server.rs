use bevy::prelude::*;

use crate::{
    net::{
        client::{self, handle_server_messages},
        server::{self, handle_client_messages},
    },
    statistics::NetState,
};

pub struct SimpleWarfareHostServerPlugin;

impl Plugin for SimpleWarfareHostServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(NetState::HostServer),
            (server::start_listening, client::start_connection),
        )
        .add_systems(
            Update,
            (handle_server_messages, handle_client_messages).run_if(in_state(NetState::HostServer)),
        );
    }
}
