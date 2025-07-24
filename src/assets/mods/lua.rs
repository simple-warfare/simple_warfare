use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;

#[derive(Debug, Default, Asset, TypePath, Clone)]
pub struct LuaAsset {
    pub context: String,
    pub crc32: u32,
}

impl LuaAsset {
    pub fn new(context: String) -> Self {
        let crc32 = crc32fast::hash(context.as_bytes());
        Self { context, crc32 }
    }
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
        Ok(Self::Asset::new(context))
    }

    fn extensions(&self) -> &[&str] {
        &["lua"]
    }
}
