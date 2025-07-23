use bevy::{platform::collections::HashMap, prelude::*};
use bevy_quinnet::{
    server::{
        ConnectionEvent, QuinnetServer, ServerEndpointConfiguration,
        certificate::CertificateRetrievalMode,
    },
    shared::ClientId,
};

use crate::{
    net::{
        protocol::{ClientMessage, ServerChannel, ServerMessage},
        shared::{LOCAL_BIND_IP, Player, SERVER_HOST, SERVER_PORT},
    },
    statistics::{GameInfo, NetState},
};

pub struct SimpleWarfareServerPlugin;

#[derive(Resource, Debug, Clone, Default)]
pub(crate) struct Players {
    pub map: HashMap<ClientId, Player>,
}

impl Plugin for SimpleWarfareServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Players>().add_systems(
            Update,
            handle_client_messages.run_if(in_state(NetState::Server)),
        );
    }
}

pub(crate) fn start_listening(mut server: ResMut<QuinnetServer>) {
    server
        .start_endpoint(
            ServerEndpointConfiguration::from_ip(LOCAL_BIND_IP, SERVER_PORT),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: SERVER_HOST.to_string(),
            },
            ServerChannel::channels_configuration(),
        )
        .unwrap();
}

pub(crate) fn handle_client_messages(
    mut server: ResMut<QuinnetServer>,
    mut players: ResMut<Players>,
    self_game_info: Res<GameInfo>,
) -> Result {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some((_, message)) = endpoint.try_receive_message_from::<ClientMessage>(client_id)
        {
            match message {
                ClientMessage::InitClient {
                    game_info,
                    player_info,
                } => {
                    if game_info == *self_game_info {
                        endpoint.send_message(client_id, ServerMessage::init_client(client_id))?;
                        endpoint.send_group_message(
                            endpoint.clients().iter(),
                            ServerMessage::new_client(client_id, player_info.clone()),
                        )?;

                        players.map.insert(client_id, player_info);
                    } else {
                        endpoint.send_message(
                            client_id,
                            ServerMessage::disconnect_client("game version is different"),
                        )?;
                    }
                }
                ClientMessage::SpawnUnit { unit_str } => {}
                ClientMessage::FetchModSet => todo!(),
            }
            
        }
    }

    Ok(())
}

pub(crate) fn handle_server_events(
    mut commands: Commands,
    mut connection_events: EventReader<ConnectionEvent>,
    mut server: ResMut<QuinnetServer>,
    mut players: ResMut<Players>,
) {
}
