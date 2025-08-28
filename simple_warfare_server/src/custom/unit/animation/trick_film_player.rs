use crate::bevy_ext::prelude::*;
use bevy::prelude::*;
use simple_warfare_server_macros::TryFromAndIntoJs;

use serde::{Deserialize, Serialize};
#[derive(Debug, Component, Clone, Deserialize, Serialize, PartialEq, Reflect, TryFromAndIntoJs)]
pub struct TrickFilmPlayer {
    #[boa(
        from_js_with = "entity_try_from_js",
        into_js_with = "entity_try_into_js"
    )]
    pub entity: Entity,

    #[boa(rename = "trickFilm")]
    pub trick_film: Option<String>,

    #[boa(rename = "trickFilmRegistion")]
    pub trick_film_registion: Option<Vec<String>>,
}
