use bevy::prelude::*;
use lightyear::prelude::*;

use crate::{
    net::{
        common::server::CommonServer,
        shared::{SEND_INTERVAL, SERVER_PORT, SHARED_SETTINGS},
    },
    statistics::NetState,
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(NetState::Server), init_server)
            .add_observer(handle_new_client);
    }
}

pub(super) fn init_server(mut commnads: Commands) {
    info!("init_server");
    commnads.spawn(CommonServer {
        conditioner: None,
        shared: SHARED_SETTINGS,
        local_port: SERVER_PORT,
    });
}

pub(crate) fn handle_new_client(trigger: Trigger<OnAdd, LinkOf>, mut commands: Commands) {
    info!("handle_new_client");
    commands
        .entity(trigger.target())
        .insert(ReplicationSender::new(
            SEND_INTERVAL,
            SendUpdatesMode::SinceLastAck,
            false,
        ));
}
