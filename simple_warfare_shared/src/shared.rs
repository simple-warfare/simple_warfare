//! This module contains the shared code between the client and the server.
use bevy::prelude::*;

use crate::protocol::*;

#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin);
    }
}

