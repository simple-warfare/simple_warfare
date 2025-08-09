use std::sync::{Arc, mpsc::Sender};

use boa_engine::prelude::*;

use crate::js_engine::simple_warfare_cli::{
    SwRequestEvent, server::trick_film_player::TrickFilmPlayerServer,
};

pub mod trick_film_player;

pub(super) fn init_server_objects(
    context: &mut Context,
    sw_request_sender: Arc<Sender<SwRequestEvent>>,
) -> Vec<(JsString, JsObject)> {
    let trick_film_player = TrickFilmPlayerServer::init(context, sw_request_sender);
    vec![(TrickFilmPlayerServer::NAME, trick_film_player)]
}
