pub mod client;
pub mod protocol;
pub mod server;
pub mod shared;
pub mod common;
use std::time::Duration;

use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use bevy::prelude::*;

use bevy_defer::AsyncWorld;
use lightyear::prelude::client::ClientPlugins;
use lightyear::prelude::server::ServerPlugins;

use crate::net::client::ClinetPlugin;
use crate::net::protocol::ProtocolPlugin;
use crate::net::server::ServerPlugin;
use crate::net::shared::{FIXED_TIMESTEP_HZ, SharedPlugin};
use bevy_webgate::{BevyWebServerPlugin, RouterAppExt};
pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MinimalPlugins, BevyWebServerPlugin))
            .route("/simple_warfare/clinet/get_room_info/thumbnail", axum::routing::post(get_asset_from_this))
            .add_plugins(ClientPlugins {
                tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
            })
            .add_plugins(ServerPlugins {
                tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
            })
            .add_plugins((SharedPlugin, ClinetPlugin, ServerPlugin, ProtocolPlugin));
    }
}

async fn get_asset_from_this(Path(asset_path): Path<String>) -> impl IntoResponse {
    info!("get_asset_from_this");
    Json("scores")
}
