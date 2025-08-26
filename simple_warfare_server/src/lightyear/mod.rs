pub mod server;

use bevy::prelude::*;

use self::server::SimpleWarfareServerPlugin;

pub struct SimpleWarfareLightyearPlugin;

impl Plugin for SimpleWarfareLightyearPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SimpleWarfareServerPlugin);
    }
}
