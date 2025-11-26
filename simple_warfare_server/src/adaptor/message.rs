use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use simple_warfare_shared_macros::{MessageDecode, MessageEncode};

use crate::consts::GAME_VERSION;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClientMessageKind {
    ContentDecodeKind,
    StartServer,
    CrateRoom,
    GetServerInfo,
    GetMapInfos,
    GetMapPaths,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ServerMessageKind {
    ServerInfo,
    ServerStarted,
    MapPaths,
    ErrorClientMessage
}

#[derive(Debug, Deserialize, Serialize, Clone, MessageDecode, MessageEncode)]
pub struct ClientMessage {
    pub kind: ClientMessageKind,
    pub content: Option<ClientMessageContent>,
}

#[derive(Debug, Deserialize, Serialize, Clone, MessageDecode, MessageEncode)]
pub struct ServerMessage {
    pub kind: ServerMessageKind,
    pub content: Option<ServerMessageContent>,
}

impl ServerMessage {
    pub fn server_info() -> Self {
        Self {
            kind: ServerMessageKind::ServerInfo,
            content: Some(ServerMessageContent::ServerInfo {
                game_version: GAME_VERSION.to_string(),
            }),
        }
    }

    pub fn started_server() -> Self {
        Self {
            kind: ServerMessageKind::ServerStarted,
            content: None,
        }
    }

    pub fn map_paths(map_paths: &Vec<String>) -> Self {
        Self {
            kind: ServerMessageKind::MapPaths,
            content: Some(ServerMessageContent::map_paths(map_paths)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ServerMessageContent {
    ServerInfo { game_version: String },
    CrateRoom {},
    MapPaths { map_paths: Vec<String> },
}

impl ServerMessageContent {
    pub fn map_paths(map_paths: &Vec<String>) -> Self {
        Self::MapPaths {
            map_paths: map_paths.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClientMessageContent {
    GetMapPaths { mod_uuid: String },
    GerAllMapPaths
}

#[derive(Debug, Event, Clone)]
pub struct ClientMessageEvent {
    pub message: ClientMessage,
}
