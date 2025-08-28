use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ClientMessageKind {
    StartServer,
    CrateRoom,
    GetServerInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ServerMessageKind {
    ServerInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClientMessage {
    pub kind: ClientMessageKind,
    pub content: Option<ClientMessageContent>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerMessage {
    pub kind: ServerMessageKind,
    pub content: Option<ServerMessageContent>,
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
