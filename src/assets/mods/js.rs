use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use mlua::FromLua;
use thiserror::Error;

#[derive(Debug, Default, Asset, TypePath, Clone, FromLua)]
pub struct JsAsset {
    pub file_name: String,
    pub context: String,
    pub from: String,
}

#[derive(Debug, Error)]
pub enum JsAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not get the file name")]
    NotFileName,
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
        if let Some(file_name) = load_context.path().file_name() {
            reader.read_to_string(&mut context).await?;

            Ok(Self::Asset {
                file_name: file_name.to_string_lossy().to_string(),
                context,
                ..Default::default()
            })
        } else {
            Err(Self::Error::NotFileName)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["js"]
    }
}
