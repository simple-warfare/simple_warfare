use crate::{add_field_function_fields, add_field_method_fields, assets::mods::js::JsAsset};
use bevy::ecs::resource::Resource;
use bevy_northstar::path;
use mlua::{FromLua, MetaMethod, UserData, UserDataFields, UserDataMethods};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, FromLua)]
pub struct ModManager {
    pub enables: Vec<ModEnableClassesDefine>,
}

impl ModManager {
    pub const LUA_GLOBAL_NAME: &'static str = "mod_manager";
}

#[derive(Debug, Deserialize, Serialize, Clone, FromLua)]
pub struct ModEnableClassesDefine {
    pub js_file_path: String,
    pub classes: Vec<String>,
}

impl UserData for ModEnableClassesDefine {}

impl ModEnableClassesDefine {
    pub fn new(js_file_path: String, classes: Vec<String>) -> Self {
        Self {
            js_file_path,
            classes,
        }
    }
}

#[derive(Debug, Default, Clone, FromLua)]
pub struct ModEnableClasses {
    pub enables: Vec<(JsAsset, Vec<String>)>,
}

impl ModEnableClasses {
    pub fn new(enables: Vec<(JsAsset, Vec<String>)>) -> Self {
        Self { enables }
    }
}

impl ModManager {
    pub fn new(enables: Vec<ModEnableClassesDefine>) -> Self {
        Self { enables }
    }
}
impl UserData for ModManager {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        add_field_method_fields!(fields { enables });
        add_field_function_fields!(fields { enables });
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut(
            "add_units",
            |_, ud, (js_file_path, classes): (String, Vec<String>)| {
                ud.enables
                    .push(ModEnableClassesDefine::new(js_file_path, classes));
                Ok(())
            },
        );
        // Constructor
        methods.add_meta_function(MetaMethod::Call, |_, ()| Ok(ModManager::default()));
        methods.add_meta_method(MetaMethod::ToString, |lua, this, ()| {
            lua.create_string(format!("{this:#?}"))
        });
    }
}

#[derive(Debug, Resource, Default, Deserialize, Serialize, Clone, FromLua)]
pub struct MapManager {
    pub map_paths: Vec<String>,
}

impl MapManager {
    pub const LUA_GLOBAL_NAME: &'static str = "map_manager";
}

impl UserData for MapManager {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        add_field_method_fields!(fields { map_paths });
        add_field_function_fields!(fields { map_paths });
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("add_maps", |_, ud, map_paths: Vec<String>| {
            map_paths.into_iter().for_each(|map_path| {
                ud.map_paths.push(map_path);
            });
            Ok(())
        });
        // Constructor
        methods.add_meta_function(MetaMethod::Call, |_, ()| Ok(MapManager::default()));
        methods.add_meta_method(MetaMethod::ToString, |lua, this, ()| {
            lua.create_string(format!("{this:#?}"))
        });
    }
}

#[derive(Debug, Resource, Default, Deserialize, Serialize, Clone, FromLua)]
pub struct NavigatorLayerManager {
    pub layers_path: Vec<String>,
}

impl NavigatorLayerManager {
    pub const LUA_GLOBAL_NAME: &'static str = "navigator_layer_manager";
}

impl UserData for NavigatorLayerManager {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        add_field_method_fields!(fields { layers_path });
        add_field_function_fields!(fields { layers_path });
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("add_layers", |_, ud, path: String| {
            ud.layers_path.push(path);
            Ok(())
        });
        // Constructor
        methods.add_meta_function(MetaMethod::Call, |_, ()| {
            Ok(NavigatorLayerManager::default())
        });
        methods.add_meta_method(MetaMethod::ToString, |lua, this, ()| {
            lua.create_string(format!("{this:#?}"))
        });
    }
}

#[derive(Debug, Resource, Default, Deserialize, Serialize, Clone, FromLua)]
pub struct ResourceManager {
    pub resources_path: Vec<String>,
}

impl ResourceManager {
    pub const LUA_GLOBAL_NAME: &'static str = "resource_manager";
}

impl UserData for ResourceManager {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        add_field_method_fields!(fields { resources_path });
        add_field_function_fields!(fields { resources_path });
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("add_resources", |_, ud, path: Option<String>| {
            match path {
                Some(path) => ud.resources_path.push(path),

                None => ud.resources_path.push("resources.toml".to_string()),
            }
            Ok(())
        });

        // Constructor
        methods.add_meta_function(MetaMethod::Call, |_, ()| Ok(ResourceManager::default()));
        methods.add_meta_method(MetaMethod::ToString, |lua, this, ()| {
            lua.create_string(format!("{this:#?}"))
        });
    }
}
