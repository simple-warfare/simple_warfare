use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, LoadedFolder, io::Reader, uuid::Uuid},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize, Default, Asset, TypePath, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub version: String,
    pub game_version: String,
    pub author: Vec<String>,
    pub description: String,
    pub uuid: Uuid,
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
    type Asset = ServerConfig;

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
        Ok(toml::from_str(&context).expect("parse mod info error"))
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
    #[error("Could not parse TOML: {0}")]
    De(#[from] serde_json::error::Error),
}

#[derive(Default)]
pub struct ModInfoJsonLoader;

impl AssetLoader for ModInfoJsonLoader {
    type Asset = ServerConfig;

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
        Ok(serde_json::from_str(&context).expect("parse mod info error"))
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
