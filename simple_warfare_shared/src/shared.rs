//! This module contains the shared code between the client and the server.
use std::time::Duration;

use bevy::prelude::*;

use crate::protocol::*;
pub const FIXED_TIMESTEP_HZ: f64 = 64.0;

pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin);
    }
}

