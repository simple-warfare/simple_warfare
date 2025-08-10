pub mod fs;
pub mod http;

use bevy::prelude::*;

use crate::js_engine::simple_warfare_cli::io::{fs::plugin::FsPlugin, http::plugin::HttpPlugin};

pub struct IoPlugin;

impl Plugin for IoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FsPlugin).add_plugins(HttpPlugin);
    }
}
