use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use lightyear::{
    netcode::NetcodeServer,
    prelude::{
        server::{NetcodeConfig, ServerUdpIo, Start},
        *,
    },
};
use simple_warfare_shared::prelude::{Lobbies, Lobby};

use crate::statistics::ServerState;

pub struct SimpleWarfareServerPlugin;

impl Plugin for SimpleWarfareServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RoomPlugin)
            .add_systems(OnEnter(ServerState::Starting), starting_server)
            .add_observer(staring_lobby);
    }
}

pub fn starting_server(mut commands: Commands) -> Result {
    let server = commands
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000)),
            ServerUdpIo::default(),
        ))
        .id();
    commands.trigger_targets(Start, server);
    Ok(())
}

pub fn staring_lobby(_trigger: Trigger<Start>, mut commands: Commands) {
    let lobbies = Lobbies::default();
    commands.spawn((lobbies, Replicate::to_clients(NetworkTarget::All)));
}
