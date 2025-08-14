use bevy::prelude::*;

use crate::{
    net::{client, server},
    statistics::NetState,
};

pub struct SimpleWarfareHostServerPlugin;

impl Plugin for SimpleWarfareHostServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(NetState::HostServer),
            (server::start_listening, client::start_connection),
        );
    }
}
