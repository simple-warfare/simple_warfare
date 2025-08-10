pub mod plugins;
use std::sync::{Arc, mpsc::Sender};

use boa_engine::prelude::*;

use crate::js_engine::simple_warfare_cli::
    server::trick_film_player::TrickFilmPlayerServer
;
use bevy::prelude::*;

use self::{plugins::TrickFilmPlayerPlugin, trick_film_player::SwTrickFilmPlayerRequestEvent};
pub mod trick_film_player;

pub(super) fn init_server_objects(
    context: &mut Context,
    sw_trick_film_player_request_sender: Arc<Sender<SwTrickFilmPlayerRequestEvent>>,
) -> Vec<(JsString, JsObject)> {
    let trick_film_player =
        TrickFilmPlayerServer::init(context, sw_trick_film_player_request_sender);
    vec![(TrickFilmPlayerServer::NAME, trick_film_player)]
}

pub struct SwServerPlugin;

impl Plugin for SwServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TrickFilmPlayerPlugin);
    }
}
