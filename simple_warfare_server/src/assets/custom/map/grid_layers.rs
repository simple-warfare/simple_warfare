use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::custom::map::navigator_layer::northstar::CustomGridLayer;

#[derive(Debug, Error)]
pub enum CustomGridLayersLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    /// A deserialization error
    #[error("Could not parse TOML: {0}")]
    De(#[from] toml::de::Error),
}

#[derive(Debug, Asset, TypePath, Default, Clone, Deserialize, Serialize)]
pub struct CustomGridLayers {
    pub layer: Vec<CustomGridLayer>,
}

#[derive(Default)]
pub struct CustomGridLayersLoader;

impl AssetLoader for CustomGridLayersLoader {
    type Asset = CustomGridLayers;

    type Settings = ();

    type Error = CustomGridLayersLoaderError;

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
