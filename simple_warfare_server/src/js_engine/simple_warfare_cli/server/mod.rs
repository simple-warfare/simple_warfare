pub mod plugins;

use boa_engine::prelude::*;

use bevy::prelude::*;

use self::plugins::TrickFilmPlayerPlugin;

pub(super) fn init_server_objects(context: &mut Context) -> Vec<(JsString, JsObject)> {
    vec![]
}

pub struct SwServerPlugin;

impl Plugin for SwServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TrickFilmPlayerPlugin);
    }
}
