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
        let file_name = load_context
            .path()
            .file_name()
            .ok_or(Self::Error::FileNameNotFound(
                load_context.path().display().to_string(),
            ))?
            .to_string_lossy()
            .into_owned();

        let from = load_context
            .path()
            .parent()
            .ok_or(Self::Error::FileNameNotFound(
                load_context.path().display().to_string(),
            ))?
            .to_string_lossy()
            .into_owned();

        let mut context = String::new();
        reader.read_to_string(&mut context).await?;

        Ok(Self::Asset {
            file_name,
            context,
            from
        })
    }

    fn extensions(&self) -> &[&str] {
        &["js", "mjs"]
    }
}
