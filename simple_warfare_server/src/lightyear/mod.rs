pub mod server;

use bevy::prelude::*;

use self::server::ServerPlugin;

pub struct LightyearPlugin;

impl Plugin for LightyearPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ServerPlugin);
    }
}
