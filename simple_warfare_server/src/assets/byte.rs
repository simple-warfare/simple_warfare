use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;

#[derive(Debug, Default, Asset, TypePath, Clone)]
pub struct ByteFile {
    pub data: Vec<u8>,
}

impl ByteFile {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

#[derive(Debug, Error)]
pub enum ByteFileLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default)]
pub struct ByteFileLoader;

impl AssetLoader for ByteFileLoader {
    type Asset = ByteFile;

    type Settings = ();

    type Error = ByteFileLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut context = Vec::new();

        reader.read_to_end(&mut context).await?;

        Ok(Self::Asset::new(context))
    }
}
