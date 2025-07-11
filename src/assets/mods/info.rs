use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, LoadedFolder, io::Reader},
    prelude::*,
};
use mlua::{FromLua, MetaMethod, UserData, UserDataFields, UserDataMethods};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{add_field_function_fields, add_field_method_fields, assets::mods::js::JsAsset};

#[derive(Debug, Deserialize, Serialize, Default, Asset, TypePath, Clone, FromLua)]
pub struct ModInfo {
    pub name: String,
    pub version: String,
    pub game_version: String,
    pub author: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, FromLua)]
pub struct ModEnableLua {
    pub enable: Vec<ModClassLua>,
}

#[derive(Debug, Deserialize, Serialize, Clone, FromLua)]
pub struct ModClassLua {
    pub js_file: String,
    pub classes: Vec<String>,
}

impl ModClassLua {
    pub fn new(js_file: String, classes: Vec<String>) -> Self {
        Self { js_file, classes }
    }
}

impl UserData for ModClassLua {}

#[derive(Debug, Default, Clone, FromLua)]
pub struct ModEnable {
    pub enable: Vec<(JsAsset, Vec<String>)>,
}

impl ModEnable {
    pub fn new(enable: Vec<(JsAsset, Vec<String>)>) -> Self {
        Self { enable }
    }
}

impl ModEnableLua {
    pub fn new(enable: Vec<ModClassLua>) -> Self {
        Self { enable }
    }
}
impl UserData for ModEnableLua {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        add_field_method_fields!(fields { enable });
        add_field_function_fields!(fields { enable });
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut(
            "enable_js",
            |_, ud, (js_file, classes): (String, Vec<String>)| {
                ud.enable.push(ModClassLua::new(js_file, classes));
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
impl UserData for ModInfo {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        add_field_method_fields!(fields {
            name,
            version,
            game_version,
            author,
        });
        add_field_function_fields!(fields {
            name,
            version,
            game_version,
            author,
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        // Constructor
        methods.add_meta_function(MetaMethod::Call, |_, ()| Ok(ModInfo::default()));
        methods.add_meta_method(MetaMethod::ToString, |lua, this, ()| {
            lua.create_string(format!("{:#?}", this))
        });
    }
}

#[derive(Debug, Error)]
pub enum ModInfoLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    /// A deserialization error
    #[error("Could not parse TOML: {0}")]
    De(#[from] toml::de::Error),
}

#[derive(Default)]
pub struct ModInfoLoader;

impl AssetLoader for ModInfoLoader {
    type Asset = ModInfo;

    type Settings = ();

    type Error = ModInfoLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut context = String::new();
        reader.read_to_string(&mut context).await?;
        if !context.contains("enable") {
            context.push_str("\nenable = []");
        }
        Ok(toml::from_str(&context).unwrap_or_default())
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
#[derive(Resource, Default)]
pub struct ModsFolderHandle(pub Handle<LoadedFolder>);
