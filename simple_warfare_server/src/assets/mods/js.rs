use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use mlua::FromLua;
use thiserror::Error;

#[derive(Debug, Default, Asset, TypePath, Clone, FromLua)]
pub struct JsAsset {
    pub path: String,
    pub context: String,
    pub crc32: u32,
}

impl JsAsset {
    pub fn new(path: impl Into<String>, context: String) -> Self {
        let crc32 = crc32fast::hash(context.as_bytes());
        Self {
            path: path.into(),
            context,
            crc32,
        }
    }
}

#[derive(Debug, Error)]
pub enum JsAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not get the file name: {0}")]
    FileNameNotFound(String),
}

#[derive(Default)]
pub struct JsAssetLoader;

impl AssetLoader for JsAssetLoader {
    type Asset = JsAsset;

    type Settings = ();

    type Error = JsAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut context = String::new();
        reader.read_to_string(&mut context).await?;

        Ok(Self::Asset::new(
            load_context.path().to_string_lossy(),
            context,
        ))
    }

    fn extensions(&self) -> &[&str] {
        &["js", "mjs"]
    }
}
