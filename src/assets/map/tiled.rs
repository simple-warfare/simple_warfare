use std::path::PathBuf;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use bevy_ecs_tiled::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Asset, TypePath)]
pub struct TiledMap {
    pub thumbnail: Option<Handle<Image>>,
    pub info: Handle<TiledMapInfo>,
    path: PathBuf,
    pub map: Option<TiledMapHandle>,
}
#[derive(Debug, Error)]
pub enum TiledMapLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not get the file name: {0}")]
    FileNameNotFound(String),
    #[error("Could not get the parent: {0}")]
    ParentPathNotFound(String),
}

#[derive(Asset, Deserialize, Serialize, TypePath)]
pub struct TiledMapInfo {
    pub title: String,
    pub description: String,
    pub author: String,
}

#[derive(Default)]
pub struct TiledMapLoader;

impl AssetLoader for TiledMapLoader {
    type Asset = TiledMap;

    type Settings = ();

    type Error = TiledMapLoaderError;

    async fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let path = load_context.path().to_owned();

        let file_name = path
            .file_stem()
            .ok_or_else(|| {
                Self::Error::FileNameNotFound(load_context.path().display().to_string())
            })?
            .to_string_lossy()
            .to_string();

        let parent_path = path.parent().ok_or_else(|| {
            Self::Error::ParentPathNotFound(load_context.path().display().to_string())
        })?;

        let info_path = parent_path.join(&file_name).join(".toml");
        let thumbnail_path = parent_path.join(&file_name).join(".png");

        let info = load_context.load(info_path);

        let thumbnail = if thumbnail_path.exists() {
            Some(load_context.load(&*thumbnail_path))
        } else {
            None
        };

        Ok(Self::Asset {
            thumbnail: thumbnail,
            path,
            map: None,
            info,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}

#[derive(Debug, Error)]
pub enum TiledMapInfoLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse file: {0}")]
    Parse(#[from] toml::de::Error),
}

pub struct TiledMapInfoLoader;

impl AssetLoader for TiledMapInfoLoader {
    type Asset = TiledMapInfo;

    type Settings = ();

    type Error = TiledMapInfoLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data);

        Ok(toml::from_slice(&data)?)
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
