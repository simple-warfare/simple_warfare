pub mod client;
pub mod host_server;
pub mod protocol;
pub mod server;
pub mod shared;
pub mod web_asset;
use bevy::prelude::*;
use bevy_quinnet::{client::QuinnetClientPlugin, server::QuinnetServerPlugin};

use crate::net::{
    client::SimpleWarfareClientPlugin, host_server::SimpleWarfareHostServerPlugin,
    server::SimpleWarfareServerPlugin, web_asset::SimpleWarfareWebAssetPlugin,
};
pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            QuinnetClientPlugin::default(),
            QuinnetServerPlugin::default(),
        ))
        .add_plugins(SimpleWarfareWebAssetPlugin)
        .add_plugins((
            SimpleWarfareServerPlugin,
            SimpleWarfareClientPlugin,
            SimpleWarfareHostServerPlugin,
        ));
    }
}
