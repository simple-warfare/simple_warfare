use std::path::Path;

use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use simple_warfare_macros::TryFromAndIntoJs;
use thiserror::Error;

#[derive(Debug, Default, Asset, TypePath, Clone, TryFromAndIntoJs)]
pub struct SectionFile {
    pub data: String,
    #[boa(rename = "realPath")]
    pub path: String,
    #[boa(rename = "realParentPath")]
    pub parent_path: String,
    pub crc32: u32,
}

impl SectionFile {
    pub fn new(data: impl Into<String>, path: &Path) -> Self {
        let data = data.into();
        let crc32 = crc32fast::hash(data.as_bytes());
        Self {
            data,
            crc32,
            path: path.to_string_lossy().into(),
            parent_path: path.parent().unwrap().to_string_lossy().into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum SectionFileLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Default)]
pub struct SectionFileLoader;

impl AssetLoader for SectionFileLoader {
    type Asset = SectionFile;

    type Settings = ();

    type Error = SectionFileLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut context = String::new();
        let path = load_context.path();
        reader.read_to_string(&mut context).await?;
        Ok(Self::Asset::new(context, path))
    }

    fn extensions(&self) -> &[&str] {
        &["section", "section.toml"]
    }
}
