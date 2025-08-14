use bevy_quinnet::shared::{
    ClientId,
    channels::{ChannelId, ChannelKind, ChannelsConfiguration, DEFAULT_MAX_RELIABLE_FRAME_LEN},
};
use serde::{Deserialize, Serialize};

use crate::{
    net::shared::{Player, UnitId},
    statistics::GameInfo,
};

use bevy::prelude::*;
use lightyear::prelude::*;

// Messages from clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    InitClient {
        //mod_set: ModSet,
        game_info: GameInfo,
        player_info: Player,
        //package_list:Vec<>
    },
    VerifyMods,
    FetchMods {
        mods: Vec<String>,
    },
    SpawnUnit {
        unit_str: String,
    },
}

impl ClientMessage {
    pub fn spawn_unit(unit_str: String) -> Self {
        Self::SpawnUnit { unit_str }
    }
}

// Messages from the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    InitClient {
        client_id: ClientId,
    },
    NewClient {
        client_id: ClientId,
        player_info: Player,
    },
    StartGame,

    VerifyMods {
        mod_js_crc32: Vec<(String, u32)>,
    },
    SpawnUnit {
        unit_id: UnitId,
        client_id: ClientId,
        unit_str: String,
    },
    DisconnectClient {
        info: String,
    },
}

impl ServerMessage {
    pub fn init_client(client_id: ClientId) -> Self {
        Self::InitClient { client_id }
    }

    pub fn new_client(client_id: ClientId, player_info: Player) -> Self {
        Self::NewClient {
            client_id,
            player_info,
        }
    }

    pub fn disconnect_client(info: impl Into<String>) -> Self {
        Self::DisconnectClient { info: info.into() }
    }

    pub fn verify_mods(mod_js_crc32: Vec<(String, u32)>) -> Self {
        Self::VerifyMods { mod_js_crc32 }
    }

    pub fn spawn_unit(client_id: ClientId, unit_id: UnitId, unit_str: String) -> Self {
        Self::SpawnUnit {
            client_id,
            unit_id,
            unit_str,
        }
    }
}

#[repr(u8)]
pub enum ClientChannel {
    GameSetup,
    GameEvents,
}
impl From<ClientChannel> for ChannelId {
    fn from(val: ClientChannel) -> Self {
        val as ChannelId
    }
}
impl ClientChannel {
    pub fn channels_configuration() -> ChannelsConfiguration {
        ChannelsConfiguration::from_types(vec![ChannelKind::default()]).unwrap()
    }
}

#[repr(u8)]
pub enum ServerChannel {
    GameSetup,
    GameEvents,
}
impl From<ServerChannel> for ChannelId {
    fn from(val: ServerChannel) -> Self {
        val as ChannelId
    }
}
impl ServerChannel {
    pub fn channels_configuration() -> ChannelsConfiguration {
        ChannelsConfiguration::from_types(vec![
            ChannelKind::OrderedReliable {
                max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
            },
            ChannelKind::UnorderedReliable {
                max_frame_size: DEFAULT_MAX_RELIABLE_FRAME_LEN,
            },
            ChannelKind::Unreliable,
        ])
        .unwrap()
    }
}

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // components
        app.register_component::<PlayerId>()
            .add_prediction(PredictionMode::Once)
            .add_interpolation(InterpolationMode::Once);

        app.register_component::<Transform>()
            .add_prediction(PredictionMode::Full)
            .add_interpolation(InterpolationMode::Full)
            .add_linear_interpolation_fn();
    }
}
