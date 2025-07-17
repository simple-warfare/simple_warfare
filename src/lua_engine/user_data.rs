use crate::{
    add_field_function_fields, add_field_method_fields,
    assets::mods::{info::ModInfo, js::JsAsset},
};
use bevy::ecs::resource::Resource;
use mlua::{FromLua, MetaMethod, UserData, UserDataFields, UserDataMethods};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, FromLua)]
pub struct ModManager {
    pub enables: Vec<ModEnableClassesDefine>,
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
            "add_js",
            |_, ud, (js_file_path, classes): (String, Vec<String>)| {
                ud.enables
                    .push(ModEnableClassesDefine::new(js_file_path, classes));
                Ok(())
            },
        );
        // Constructor
        methods.add_meta_function(MetaMethod::Call, |_, ()| Ok(ModInfo::default()));
        methods.add_meta_method(MetaMethod::ToString, |lua, this, ()| {
            lua.create_string(format!("{:#?}", this))
        });
    }
}

#[derive(Debug, Resource, Default, Deserialize, Serialize, Clone, FromLua)]
pub struct MapManager {
    pub map_paths: Vec<String>,
}

impl UserData for MapManager {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        add_field_method_fields!(fields { map_paths });
        add_field_function_fields!(fields { map_paths });
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("add_map", |_, ud, map_path: String| {
            ud.map_paths.push(map_path);
            Ok(())
        });
        // Constructor
        methods.add_meta_function(MetaMethod::Call, |_, ()| Ok(ModInfo::default()));
        methods.add_meta_method(MetaMethod::ToString, |lua, this, ()| {
            lua.create_string(format!("{:#?}", this))
        });
    }
}
