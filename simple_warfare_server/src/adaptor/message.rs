use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use simple_warfare_shared_macros::{MessageDecode, MessageEncode};
use thiserror::Error;

use crate::consts::GAME_VERSION;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClientMessageKind {
    ContentDecodeKind,
    StartServer,
    CrateRoom,
    GetServerInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ServerMessageKind {
    ServerInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone, MessageDecode, MessageEncode)]
pub struct ClientMessage {
    pub kind: ClientMessageKind,
    pub content: Option<ServerMessageContent>,
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
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ServerMessageContent {
    ServerInfo { game_version: String },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClientMessageContent {}

#[derive(Debug, Event, Clone)]
pub struct ClientMessageEvent {
    pub client: Entity,
    pub message: ClientMessage,
}
