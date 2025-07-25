use bevy::{platform::collections::HashMap, prelude::*};
use bevy_quinnet::{
    server::{
        ConnectionEvent, QuinnetServer, ServerEndpointConfiguration,
        certificate::CertificateRetrievalMode,
    },
    shared::ClientId,
};

use crate::{
    assets::{
        GameAsset,
        mods::{info::ModInfo, js::JsAsset, lua::LuaAsset},
    },
    net::{
        protocol::{ClientMessage, ServerChannel, ServerMessage},
        shared::{LOCAL_BIND_IP, Player, SERVER_HOST, SERVER_PORT},
    },
    statistics::{GameInfo, NetState},
};

pub struct SimpleWarfareServerPlugin;

#[derive(Resource, Debug, Clone, Default)]
pub struct Players {
    pub map: HashMap<ClientId, Player>,
}

#[derive(Debug, Resource)]
pub struct ServerData {
    pub mod_js_crc32: Vec<(String, u32)>,
}

impl Plugin for SimpleWarfareServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Players>().add_systems(
            Update,
            (handle_client_messages, handle_server_events)
                .run_if(in_state(NetState::Server).or(in_state(NetState::HostServer))),
        );
    }
}

pub fn start_listening(
    mut commands: Commands,
    mut server: ResMut<QuinnetServer>,
    mut game_asset: ResMut<GameAsset>,
    js_assets: Res<Assets<JsAsset>>,
    mod_infos: Res<Assets<ModInfo>>,
    lua_assets: Res<Assets<LuaAsset>>,
) {
    let custom_mods: Vec<crate::custom::CustomModAsset> = game_asset
        .custom_mod_handles
        .mod_handles
        .iter()
        .map(|custom_mod_handle| custom_mod_handle.to_asset(&js_assets, &mod_infos, &lua_assets))
        .collect();

    commands.insert_resource(ServerData {
        mod_js_crc32: custom_mods
            .iter()
            .flat_map(|custom_mod_asset| {
                custom_mod_asset
                    .custom_mod_enable_js
                    .iter()
                    .map(|custom_mod_enable_js| {
                        (
                            custom_mod_enable_js.js_asset.path.clone(),
                            custom_mod_enable_js.js_asset.crc32,
                        )
                    })
            })
            .collect(),
    });

    game_asset.custom_mods = Some(custom_mods);

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

pub fn handle_client_messages(
    mut server: ResMut<QuinnetServer>,
    mut players: ResMut<Players>,
    self_game_info: Res<GameInfo>,
    server_data: Res<ServerData>,
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
                ClientMessage::VerifyMods => {
                    //game_asset.custom_mods.mods.iter().map(|custom_mod|custom_mod.)
                    endpoint.send_message(
                        client_id,
                        ServerMessage::verify_mods(server_data.mod_js_crc32.clone()),
                    )?;
                }
                ClientMessage::FetchMods { mods } => todo!(),
            }
        }
    }

    Ok(())
}

pub fn handle_server_events(
    mut connection_events: EventReader<ConnectionEvent>,
    mut server: ResMut<QuinnetServer>,
) -> Result {
    for client in connection_events.read() {
        server
            .endpoint_mut()
            .send_message(client.id, ServerMessage::init_client(client.id))?;
    }
    Ok(())
}
