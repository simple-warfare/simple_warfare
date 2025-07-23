use bevy::prelude::{Entity, Vec2, Vec3};
use bevy_quinnet::shared::{
    ClientId,
    channels::{ChannelId, ChannelKind, ChannelsConfiguration, DEFAULT_MAX_RELIABLE_FRAME_LEN},
};
use serde::{Deserialize, Serialize};

use crate::{assets::mods::ModSet, net::shared::Player, statistics::GameInfo};

// Messages from clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ClientMessage {
    InitClient {
        //mod_set: ModSet,
        game_info: GameInfo,
        player_info: Player,
        //package_list:Vec<>
    },
    FetchModSet,
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
pub(crate) enum ServerMessage {
    InitClient {
        client_id: ClientId,
    },
    NewClient {
        client_id: ClientId,
        player_info: Player,
    },
    StartGame,
    PushModSet {
        mod_set: ModSet,
    },
    SpawnUnit {
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
}

#[repr(u8)]
pub enum ClientChannel {
    GameSetup,
    GameEvents,
}
impl Into<ChannelId> for ClientChannel {
    fn into(self) -> ChannelId {
        self as ChannelId
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
impl Into<ChannelId> for ServerChannel {
    fn into(self) -> ChannelId {
        self as ChannelId
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
