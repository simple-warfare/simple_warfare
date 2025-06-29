use std::path::Path;

use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use bevy_ecs_ldtk::{ldtk::LdtkJson, prelude::*};
use thiserror::Error;

#[derive(Asset, TypePath)]
pub struct LdtkMap {
    pub thumbnail: Handle<Image>,
    pub map: LdtkProjectHandle,
}
#[derive(Debug, Error)]
pub enum LdtkMapLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not get the file name: {0}")]
    FileNameNotFound(String),
    #[error("Could not get the parent: {0}")]
    ParentPathNotFound(String),
    #[error("Could not parse file: {0}")]
    Json(#[from] serde_json::error::Error),
}

#[derive(Default)]
pub struct LdtkMapLoader;

impl AssetLoader for LdtkMapLoader {
    type Asset = LdtkMap;

    type Settings = ();

    type Error = LdtkMapLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        /*
        let mut context = String::new();
        reader.read_to_string(&mut context).await?;
        let mut json: LdtkJson = serde_json::from_str(&context)?;
        for level in json.levels.iter_mut() {
            if let Some(ref mut external_rel_path) = level.external_rel_path {
                let real_path = Path::new(&external_rel_path);
                let file_name = real_path
                    .file_name()
                    .ok_or_else(|| Self::Error::FileNameNotFound(external_rel_path.to_string()))?
                    .to_string_lossy();

                *external_rel_path = format!("texture/tiles/{}", file_name);
            }
        }
        */

        let asset_path = load_context.asset_path().clone();

        let file_name = load_context
            .path()
            .file_name()
            .ok_or_else(|| {
                Self::Error::FileNameNotFound(load_context.path().display().to_string())
            })?
            .to_string_lossy();

        let parent_path = load_context.path().parent().ok_or_else(|| {
            Self::Error::ParentPathNotFound(load_context.path().display().to_string())
        })?;

        let thumbnail_path = format!("{}/{}.png", parent_path.display(), file_name);

        // Await the async loading operations
        let (thumbnail, map) = (
            load_context.load(&thumbnail_path),
            load_context.load(&asset_path).into(),
        );

        Ok(Self::Asset { thumbnail, map })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
