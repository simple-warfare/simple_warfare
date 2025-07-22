pub mod web_asset;

use std::time::Duration;

use crate::net::web_asset::get_thumbnail_from_this;
use bevy::prelude::*;
use bevy_webgate::{BevyWebServerPlugin, RouterAppExt};
pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyWebServerPlugin).route(
            "/simple_warfare/clinet/get_room_info/thumbnail",
            axum::routing::get(get_thumbnail_from_this),
        );
    }
}
