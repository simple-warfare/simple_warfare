use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, LoadedFolder, io::Reader},
    prelude::*,
};
use mlua::{FromLua, MetaMethod, UserData, UserDataFields, UserDataMethods};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{add_field_function_fields, add_field_method_fields};

#[derive(Debug, Deserialize, Serialize, Default, Asset, TypePath, Clone, FromLua)]
pub struct ModInfo {
    pub name: String,
    pub version: String,
    pub game_version: String,
    pub author: String,
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
        Ok(toml::from_str(&context).unwrap_or_default())
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
#[derive(Resource, Default)]
pub struct ModSetsFolderHandle(pub Handle<LoadedFolder>);
