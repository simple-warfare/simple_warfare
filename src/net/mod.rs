pub mod client;
pub mod common;
pub mod host_server;
pub mod protocol;
pub mod server;
pub mod shared;
pub mod web_asset;

use std::time::Duration;

use bevy::prelude::*;

use lightyear::prelude::client::ClientPlugins;
use lightyear::prelude::server::ServerPlugins;

use crate::net::client::ClinetPlugin;
use crate::net::host_server::HostServerPlugin;
use crate::net::server::ServerPlugin;
use crate::net::shared::{FIXED_TIMESTEP_HZ, SharedPlugin};
use crate::net::web_asset::get_thumbnail_from_this;
use bevy_webgate::{BevyWebServerPlugin, RouterAppExt};
pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyWebServerPlugin)
            .route(
                "/simple_warfare/clinet/get_room_info/thumbnail",
                axum::routing::get(get_thumbnail_from_this),
            )
            .add_plugins(ClientPlugins {
                tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
            })
            .add_plugins(ServerPlugins {
                tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
            })
            .add_plugins((SharedPlugin, ClinetPlugin, ServerPlugin, HostServerPlugin));
    }
}
