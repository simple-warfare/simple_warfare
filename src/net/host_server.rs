use bevy::prelude::*;

use crate::{
    net::{client::init_client, server::init_server},
    statistics::NetState,
};

pub struct HostServerPlugin;

impl Plugin for HostServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(NetState::HostServer),
            (init_server, init_client).chain(),
        );
    }
}
