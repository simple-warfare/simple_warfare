use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, LoadedFolder, io::Reader, uuid::Uuid},
    prelude::*,
};
use boa_engine::value::TryIntoJs;
use mlua::{FromLua, MetaMethod, UserData, UserDataFields, UserDataMethods};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{add_field_function_fields, add_field_method_fields, bevy_ext::try_into_js::*};

#[derive(Debug, Deserialize, Serialize, Default, Asset, TypePath, Clone, FromLua, TryIntoJs)]
pub struct ModInfo {
    pub name: String,
    pub version: String,
    pub game_version: String,
    pub author: Vec<String>,
    pub description: String,
    #[boa(into_js_with = "uuid_try_into_js")]
    pub uuid: Uuid,
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
            lua.create_string(format!("{this:#?}"))
        });
    }
}

#[derive(Debug, Error)]
pub enum ModInfoTomlLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    /// A deserialization error
    #[error("Could not parse TOML: {0}")]
    De(#[from] toml::de::Error),
}

#[derive(Default)]
pub struct ModInfoTomlLoader;

impl AssetLoader for ModInfoTomlLoader {
    type Asset = ModInfo;

    type Settings = ();

    type Error = ModInfoTomlLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut context = String::new();
        reader.read_to_string(&mut context).await?;
        Ok(toml::from_str(&context)?)
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}

#[derive(Debug, Error)]
pub enum ModInfoJsonLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    /// A deserialization error
    #[error("Could not parse Json: {0}")]
    De(#[from] serde_json::error::Error),
}

#[derive(Default)]
pub struct ModInfoJsonLoader;

impl AssetLoader for ModInfoJsonLoader {
    type Asset = ModInfo;

    type Settings = ();

    type Error = ModInfoJsonLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut context = String::new();
        reader.read_to_string(&mut context).await?;
        Ok(serde_json::from_str(&context)?)
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
#[derive(Resource, Default)]
pub struct ModSetsFolderHandle(pub Handle<LoadedFolder>);
