use aeronet_websocket::server::WebSocketServerPlugin;
use bevy::prelude::*;

use simple_warfare_server::SimpleWarfareServerPlugins;
use simple_warfare_shared::SimpleWarfareSharedPlugin;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(SimpleWarfareSharedPlugin)
        .add_plugins(SimpleWarfareServerPlugins)
        .run();
}
