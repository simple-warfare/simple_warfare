use bevy::prelude::*;
use std::hash::Hash;

use crate::{
    js_engine::{plugin::BoaLoadJsAsset, sw::plugin::JsReadTomlFiles},
    mod_engine::server::ModServer,
};

pub fn pressed_button<T>(code: T) -> impl FnMut(Res<ButtonInput<T>>) -> bool + Clone
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    move |input: Res<ButtonInput<T>>| input.just_pressed(code)
}

pub fn mod_server_has_data() -> impl FnMut(Res<ModServer>) -> bool + Clone {
    move |mod_server: Res<ModServer>| !mod_server.client_messages.is_empty()
}

pub fn boa_load_js_asset_has_data() -> impl FnMut(Res<BoaLoadJsAsset>) -> bool + Clone {
    move |boa_load_js_asset: Res<BoaLoadJsAsset>| !boa_load_js_asset.map.is_empty()
}
pub fn js_read_toml_files_has_data() -> impl FnMut(Res<JsReadTomlFiles>) -> bool + Clone {
    move |js_read_toml_files: Res<JsReadTomlFiles>| !js_read_toml_files.map.is_empty()
}
