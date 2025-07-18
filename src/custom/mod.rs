use bevy::asset::Handle;

use crate::assets::{
    map::SimpleWarfareMap,
    mods::{info::ModInfo, js::JsAsset, lua::LuaAsset},
};

pub mod ui;
pub mod unit;

#[derive(Debug, Default)]
pub struct CustomMod {
    pub info: Handle<ModInfo>,
    pub main_lua: Handle<LuaAsset>,
    pub maps: Vec<SimpleWarfareMap>,
    pub enable_js: Vec<(Handle<JsAsset>, Vec<String>)>,
}

impl CustomMod {
    pub fn new(info: Handle<ModInfo>, main_lua: Handle<LuaAsset>) -> Self {
        Self {
            info,
            main_lua,
            ..Default::default()
        }
    }
}
