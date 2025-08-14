pub mod info;
pub mod js;
pub mod lua;

use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default, Asset, TypePath, Clone)]
pub struct ModSetNowUseConf {
    pub use_mod_set: String,
}

impl ModSetNowUseConf {
    pub fn new(use_mod_set: String) -> Self {
        Self { use_mod_set }
    }
}

#[derive(Debug, Error)]
pub enum ModSetNowUseLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default)]
pub struct ModSetNowUseConfLoader;

impl AssetLoader for ModSetNowUseConfLoader {
    type Asset = ModSetNowUseConf;

    type Settings = ();

    type Error = ModSetNowUseLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut use_mod_set = String::new();
        reader.read_to_string(&mut use_mod_set).await?;
        Ok(Self::Asset::new(use_mod_set))
    }

    fn extensions(&self) -> &[&str] {
        &["conf"]
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Asset, TypePath, Clone)]
pub struct ModSet {
    pub enable_mods: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ModSetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    /// A deserialization error
    #[error("Could not parse TOML: {0}")]
    De(#[from] toml::de::Error),
}

#[derive(Default)]
pub struct ModSetLoader;

impl AssetLoader for ModSetLoader {
    type Asset = ModSet;

    type Settings = ();

    type Error = ModSetLoaderError;

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
