use std::path::Path;

use bevy::prelude::*;
use bevy_quinnet::{
    client::{
        QuinnetClient, certificate::CertificateVerificationMode,
        connection::ClientEndpointConfiguration,
    },
    shared::ClientId,
};

use crate::{
    assets::mods::js::JsAsset,
    mod_engine::server::ModServer,
    net::{
        protocol::{ClientChannel, ClientMessage, ServerMessage},
        server::Players,
        shared::{LOCAL_BIND_IP, SERVER_HOST, SERVER_PORT, UnitMapping},
    },
    statistics::{NetClientState, NetState},
};

#[derive(Resource, Debug, Clone, Default)]
pub struct ClientData {
    self_id: ClientId,
    fetch_mods: Vec<String>,
    wait_ready_mods_js: Vec<(Handle<JsAsset>, u32)>,
    untyped_handles: Vec<UntypedHandle>,
}

impl ClientData {
    pub fn add_wait_ready_mod_js(&mut self, js_handle: Handle<JsAsset>, crc32: u32) {
        self.wait_ready_mods_js.push((js_handle.clone(), crc32));
        self.untyped_handles.push(js_handle.untyped());
    }
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
            )
            .add_systems(
                Update,
                check_fetch_mods.run_if(
                    in_state(NetState::Client)
                        .or(in_state(NetState::HostServer))
                        .and(in_state(NetClientState::VerifyMods)),
                ),
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
    asset_server: Res<AssetServer>,
    mut client: ResMut<QuinnetClient>,
    mut client_data: ResMut<ClientData>,
    mut players: ResMut<Players>,
    mut net_client_state: ResMut<NextState<NetClientState>>,
    mod_server: ResMut<ModServer>,
    js_assets: Res<Assets<JsAsset>>,
    unit_mapping: ResMut<UnitMapping>,
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
            info!("Client initialized with ID: {}", client_id);
            connection.send_message(ClientMessage::VerifyMods)?;
            net_client_state.set(NetClientState::VerifyMods);
        }
        ServerMessage::StartGame => {}
        ServerMessage::SpawnUnit {
            client_id,
            unit_id,
            unit_str,
        } => {
            mod_server.spawn_unit(unit_id, &unit_str);
        }
        ServerMessage::DisconnectClient { info } => {}
        ServerMessage::NewClient {
            client_id,
            player_info,
        } => {
            players.map.insert(client_id, player_info);
        }
        ServerMessage::VerifyMods { mod_js_crc32 } => {
            info!("Received mod verification data: {:?}", mod_js_crc32);
            mod_js_crc32
                .iter()
                .try_for_each::<_, Result>(|(path, js_crc32)| {
                    let local_path = Path::new(path);
                    if local_path.exists() {
                        // 从本地加载
                        let js_asset_handle = asset_server.load(local_path);
                        match js_assets.get(js_asset_handle.id()) {
                            Some(js_asset) => {
                                if js_asset.crc32 != *js_crc32 {
                                    //Js文件已加载但与服务器不一致
                                    info!(
                                        "JS asset {} loaded but CRC32 mismatch: expected {}, got {}",
                                        path, js_asset.crc32, js_crc32
                                    );
                                    client_data.fetch_mods.push(path.clone());
                                }
                            }
                            None => {
                                //Js文件存在但未被加载
                                client_data
                                    .add_wait_ready_mod_js(js_asset_handle.clone(), *js_crc32);
                            }
                        }
                    } else {
                        //从服务器加载
                        client_data.fetch_mods.push(path.clone());
                    };

                    Ok(())
                })?;
        }
    }

    Ok(())
}

pub fn check_fetch_mods(
    mut client: ResMut<QuinnetClient>,
    mut client_data: ResMut<ClientData>,
    mut net_client_state: ResMut<NextState<NetClientState>>,
    asset_server: Res<AssetServer>,
    js_assets: Res<Assets<JsAsset>>,
) -> Result {
    if client_data.fetch_mods.is_empty() && client_data.wait_ready_mods_js.is_empty() {
        net_client_state.set(NetClientState::Ready);
        info!("Client is ready, all mods are loaded.");
        return Ok(());
    }

    // 处理需要加载的JS文件
    client_data
        .untyped_handles
        .retain(|handle| !asset_server.is_loaded_with_dependencies(handle.id()));

    if client_data.wait_ready_mods_js.is_empty() {
        client_data
            .wait_ready_mods_js
            .clone()
            .iter()
            .try_for_each::<_, Result>(|(handle, crc32)| {
                if let Some(js_asset) = js_assets.get(handle.id()) {
                    if js_asset.crc32 != *crc32 {
                        //Js文件已加载但与服务器不一致
                        client_data
                            .fetch_mods
                            .push(handle.path().unwrap().to_string());
                    }
                } else {
                    return Err(BevyError::from("JS asset not found"));
                }
                Ok(())
            })?;

        // 处理需要下载的mod
        let Some(connection) = client.get_connection_mut() else {
            return Ok(());
        };

        connection.send_message(ClientMessage::FetchMods {
            mods: client_data.fetch_mods.clone(),
        })?;
        net_client_state.set(NetClientState::FetchMods);
    }

    Ok(())
}
