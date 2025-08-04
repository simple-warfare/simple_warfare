use crate::{
    assets::{
        map::tiled::SimpleWarfareMap,
        mods::{info::ModInfo, js::JsAsset, lua::LuaAsset},
    },
    custom::map::navigator_layer::{NavigatorLayerPlugin, northstar::CustomGridLayersServer},
};
use bevy::prelude::*;

pub mod map;
pub mod ui;
pub mod unit;

#[derive(Debug, Default, Clone)]
pub struct CustomModHandle {
    pub info: Handle<ModInfo>,
    pub main_lua: Handle<LuaAsset>,
    pub maps: Vec<Handle<SimpleWarfareMap>>,
    pub custom_mod_enable_js_handles: Vec<CustomModEnableJsHandle>,
}

#[derive(Debug, Default, Clone)]
pub struct CustomModAsset {
    pub info: ModInfo,
    pub main_lua: LuaAsset,
    pub custom_mod_enable_js: Vec<CustomModEnableJs>,
}

#[derive(Debug, Default, Clone)]
pub struct CustomModEnableJs {
    pub js_asset: JsAsset,
    pub enable_class: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct CustomModEnableJsHandle {
    pub js_asset: Handle<JsAsset>,
    pub enable_class: Vec<String>,
}

impl CustomModEnableJsHandle {
    pub fn new(js_asset: Handle<JsAsset>, enable_class: Vec<String>) -> Self {
        Self {
            js_asset,
            enable_class,
        }
    }
}
impl CustomModAsset {
    pub fn new(
        info: ModInfo,
        main_lua: LuaAsset,
        custom_mod_enable_js: Vec<CustomModEnableJs>,
    ) -> Self {
        Self {
            info,
            main_lua,
            custom_mod_enable_js,
        }
    }
}

impl CustomModEnableJs {
    pub fn new(js_asset: JsAsset, enable_class: Vec<String>) -> Self {
        Self {
            js_asset,
            enable_class,
        }
    }
}

impl CustomModEnableJsHandle {
    pub fn to_asset(&self, js_assets: &Res<Assets<JsAsset>>) -> CustomModEnableJs {
        CustomModEnableJs::new(
            js_assets.get(self.js_asset.id()).unwrap().clone(),
            self.enable_class.clone(),
        )
    }
}
impl CustomModHandle {
    pub fn new(info: Handle<ModInfo>, main_lua: Handle<LuaAsset>) -> Self {
        Self {
            info,
            main_lua,
            ..Default::default()
        }
    }

    pub fn to_asset(
        &self,
        js_assets: &Res<Assets<JsAsset>>,
        mod_infos: &Res<Assets<ModInfo>>,
        lua_assets: &Res<Assets<LuaAsset>>,
    ) -> CustomModAsset {
        CustomModAsset::new(
            mod_infos.get(self.info.id()).unwrap().clone(),
            lua_assets.get(self.main_lua.id()).unwrap().clone(),
            self.custom_mod_enable_js_handles
                .iter()
                .map(|custom_mod_enable_js_handle| custom_mod_enable_js_handle.to_asset(js_assets))
                .collect(),
        )
    }
}

pub struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NavigatorLayerPlugin)
            .init_resource::<CustomGridLayersServer>();
    }
}
