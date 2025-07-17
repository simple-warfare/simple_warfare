use bevy::asset::Handle;

use crate::assets::mods::{info::ModInfo, js::JsAsset, lua::LuaAsset};

pub mod ui;
pub mod unit;

#[derive(Debug)]
pub struct CustomMod {
    pub info: Handle<ModInfo>,
    pub main_lua: Handle<LuaAsset>,
    pub enable_js: Vec<(Handle<JsAsset>, Vec<String>)>,
}

impl CustomMod {
    pub fn new(info: Handle<ModInfo>, main_lua: Handle<LuaAsset>) -> Self {
        Self {
            info,
            main_lua,
            enable_js: Vec::new(),
        }
    }
}
