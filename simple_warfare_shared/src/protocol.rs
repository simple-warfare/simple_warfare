//! This file contains the shared [`Protocol`] that defines the messages that can be sent between the client and server.
//!
//! You will need to define the [`Components`], [`Messages`] and [`Inputs`] that make up the protocol.
//! You can use the `#[protocol]` attribute to specify additional behaviour:
//! - how entities contained in the message should be mapped from the remote world to the local world
//! - how the component should be synchronized between the `Confirmed` entity and the `Predicted`/`Interpolated` entity
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use lightyear::prelude::*;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Default, Reflect)]
pub struct Lobbies {
    pub lobbies: Vec<Lobby>,
}

impl Lobbies {
    /// Return true if there is an empty lobby available for players to join
    pub(crate) fn has_empty_lobby(&self) -> bool {
        if self.lobbies.is_empty() {
            return false;
        }
        self.lobbies.iter().any(|lobby| lobby.players.is_empty())
    }

    /// Remove a client from a lobby
    pub(crate) fn remove_client(&mut self, client_id: PeerId, commands: &mut Commands) {
        let mut removed_lobby = None;
        for (lobby_id, lobby) in self.lobbies.iter_mut().enumerate() {
            if let Some(index) = lobby.players.iter().position(|id| *id == client_id) {
                lobby.players.remove(index);
                if lobby.players.is_empty() {
                    removed_lobby = Some(lobby_id);
                    commands.entity(lobby.room).despawn();
                }
            }
        }
        if let Some(lobby_id) = removed_lobby {
            self.lobbies.remove(lobby_id);
            // always make sure that there is an empty lobby for players to join
            if !self.has_empty_lobby() {
                let room = commands.spawn(Room::default()).id();
                self.lobbies.push(Lobby::new(room));
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct Lobby {
    pub players: Vec<PeerId>,
    pub room: Entity,
    /// If true, the lobby is in game. If not, it is still in lobby mode
    pub in_game: bool,
}

impl Lobby {
    pub fn new(room: Entity) -> Self {
        Self {
            players: vec![],
            room,
            in_game: false,
        }
    }
}

// Channels
pub struct Channel1;

// Messages

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StartGame {
    pub(crate) lobby_id: usize,
    pub(crate) host: Option<PeerId>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExitLobby {
    pub(crate) lobby_id: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct JoinLobby {
    pub(crate) lobby_id: usize,
}

// Protocol
pub(crate) struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Lobbies>();
        // messages
        app.add_message::<StartGame>()
            .add_direction(NetworkDirection::Bidirectional);
        app.add_message::<JoinLobby>()
            .add_direction(NetworkDirection::ClientToServer);
        app.add_message::<ExitLobby>()
            .add_direction(NetworkDirection::ClientToServer);

        // channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        })
        .add_direction(NetworkDirection::Bidirectional);
    }
}
