use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;

#[derive(Debug, Default, Asset, TypePath, Clone)]
pub struct JsTomlFile {
    pub data: String,
    pub crc32: u32,
}

impl JsTomlFile {
    pub fn new(data: String) -> Self {
        let crc32 = crc32fast::hash(data.as_bytes());
        Self { data, crc32 }
    }
}

#[derive(Debug, Error)]
pub enum JsTomlFileLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default)]
pub struct JsTomlFileLoader;

impl AssetLoader for JsTomlFileLoader {
    type Asset = JsTomlFile;

    type Settings = ();

    type Error = JsTomlFileLoaderError;

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
        &["toml"]
    }
}
