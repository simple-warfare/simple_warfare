use bevy::prelude::*;

use simple_warfare_client::SimpleWarfareClientPlugins;
use simple_warfare_shared::SimpleWarfareSharedPlugin;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(SimpleWarfareSharedPlugin)
        .add_plugins(SimpleWarfareClientPlugins)
        .run();
}
