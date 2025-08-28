
use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use simple_warfare_server_macros::TryFromAndIntoJs;
use thiserror::Error;

#[derive(Debug, Default, Asset, TypePath, Clone, TryFromAndIntoJs)]
pub struct TomlFile {
    pub data: String,
    pub crc32: u32,
}

impl TomlFile {
    pub fn new(data: String) -> Self {
        let crc32 = crc32fast::hash(data.as_bytes());
        Self { data, crc32 }
    }
}

#[derive(Debug, Error)]
pub enum TomlFileLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default)]
pub struct TomlFileLoader;

impl AssetLoader for TomlFileLoader {
    type Asset = TomlFile;

    type Settings = ();

    type Error = TomlFileLoaderError;

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