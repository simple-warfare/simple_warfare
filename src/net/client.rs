use bevy::prelude::*;
use bevy_quinnet::{
    client::{
        QuinnetClient, certificate::CertificateVerificationMode,
        connection::ClientEndpointConfiguration,
    },
    shared::ClientId,
};

use crate::{
    net::{
        protocol::{ClientChannel, ClientMessage, ServerMessage},
        server::Players,
        shared::{LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT},
    },
    statistics::NetState,
};

#[derive(Resource, Debug, Clone, Default)]
pub struct ClientData {
    self_id: ClientId,
}

pub struct SimpleWarfareClientPlugin;

impl Plugin for SimpleWarfareClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientData>()
            .add_systems(OnEnter(NetState::Client), start_connection)
            .add_systems(
                Update,
                handle_server_messages
                    .run_if(in_state(NetState::Client).or(in_state(NetState::HostServer))),
            );
    }
}

pub fn start_connection(mut client: ResMut<QuinnetClient>) {
    client
        .open_connection(
            ClientEndpointConfiguration::from_ips(SERVER_HOST, SERVER_PORT, LOCAL_BIND_IP, 0),
            CertificateVerificationMode::SkipVerification,
            ClientChannel::channels_configuration(),
        )
        .unwrap();
}

pub fn handle_server_messages(
    mut client: ResMut<QuinnetClient>,
    mut client_data: ResMut<ClientData>,
    mut players: ResMut<Players>,
) -> Result {
    let Some(connection) = client.get_connection_mut() else {
        return Ok(());
    };
    let Some((_, message)) = connection.try_receive_message::<ServerMessage>() else {
        return Ok(());
    };
    match message {
        ServerMessage::InitClient { client_id } => {
            client_data.self_id = client_id;
            connection.send_message(ClientMessage::FetchMods)?;
        }
        ServerMessage::StartGame => {}
        ServerMessage::SpawnUnit {
            client_id,
            unit_str,
        } => {}
        ServerMessage::DisconnectClient { info } => {}
        ServerMessage::NewClient {
            client_id,
            player_info,
        } => {
            players.map.insert(client_id, player_info);
        }
        ServerMessage::FetchMods { js_crc32 } => {
            info!("js_crc32:{:?}", js_crc32);
        }
    }

    Ok(())
}
