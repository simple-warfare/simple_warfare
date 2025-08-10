use bevy::prelude::*;
use std::hash::Hash;

use crate::{
    js_engine::{plugin::BoaLoadJsAsset, simple_warfare_cli::io::fs::plugin::ReadFilesMap},
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
pub fn read_files_has_data<R: Resource + ReadFilesMap>() -> impl FnMut(Res<R>) -> bool + Clone {
    move |js_read_toml_files: Res<R>| !js_read_toml_files.get_map().is_empty()
}
