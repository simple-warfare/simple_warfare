pub mod fs;
pub mod http;

use bevy::prelude::*;

use crate::js_engine::simple_warfare_cli::io::{fs::plugin::SwFsPlugin, http::plugin::SwHttpPlugin};

pub struct SwIoPlugin;

impl Plugin for SwIoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SwFsPlugin).add_plugins(SwHttpPlugin);
    }
}
