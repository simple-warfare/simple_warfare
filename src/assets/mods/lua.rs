use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;

#[derive(Debug, Default, Asset, TypePath)]
pub struct LuaAsset {
    pub context: String,
}

#[derive(Debug, Error)]
pub enum LuaAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default)]
pub struct LuaAssetLoader;

impl AssetLoader for LuaAssetLoader {
    type Asset = LuaAsset;

    type Settings = ();

    type Error = LuaAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut context = String::new();
        reader.read_to_string(&mut context).await?;

        Ok(Self::Asset { context })
    }

    fn extensions(&self) -> &[&str] {
        &["lua"]
    }
}
