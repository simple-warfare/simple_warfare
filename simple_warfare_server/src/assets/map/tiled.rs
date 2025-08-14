use std::path::PathBuf;

use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use bevy_ecs_tiled::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const MISSING_MAP_THUMBNAIL_PATH: &str = "texture/interface/missing_map_thumbnail.png";

#[derive(Debug, Clone, Asset, TypePath)]
pub struct SimpleWarfareMap {
    pub map_thumbnail_handle: Handle<Image>,
    pub map_info_handle: Handle<SimpleWarfareMapInfo>,
    pub map_path: PathBuf,
    pub map_asset_handle: Option<Handle<TiledMapAsset>>,
}

impl SimpleWarfareMap {
    pub fn get_tiled_map_asset<'a>(
        &self,
        tiled_map_assets: &'a Res<Assets<TiledMapAsset>>,
    ) -> Option<&'a TiledMapAsset> {
        tiled_map_assets.get(self.map_asset_handle.as_ref().unwrap().id())
    }
    pub fn get_map_info<'a>(
        &self,
        simple_warfare_map_infos: &'a Res<Assets<SimpleWarfareMapInfo>>,
    ) -> Option<&'a SimpleWarfareMapInfo> {
        simple_warfare_map_infos.get(self.map_info_handle.id())
    }
    pub fn get_map_thumbnail<'a>(
        &self,
        simple_warfare_map_thumbnails: &'a Res<Assets<Image>>,
    ) -> Option<&'a Image> {
        simple_warfare_map_thumbnails.get(self.map_thumbnail_handle.id())
    }
}

#[derive(Debug, Error)]
pub enum SimpleWarfareMapLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not get the file name: {0}")]
    FileNameNotFound(String),
    #[error("Could not get the parent: {0}")]
    ParentPathNotFound(String),
}

#[derive(Default)]
pub struct SimpleWarfareMapLoader;

impl AssetLoader for SimpleWarfareMapLoader {
    type Asset = SimpleWarfareMap;

    type Settings = ();

    type Error = SimpleWarfareMapLoaderError;

    async fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let map_path = load_context.path().to_owned();

        let file_stem = map_path
            .file_stem()
            .ok_or(Self::Error::FileNameNotFound(
                load_context.path().display().to_string(),
            ))?
            .to_string_lossy()
            .to_string();

        let parent_path = map_path.parent().ok_or(Self::Error::ParentPathNotFound(
            load_context.path().display().to_string(),
        ))?;

        let map_info_path = parent_path.join(format!("{file_stem}.toml"));
        let map_thumbnail_path = parent_path.join(format!("{file_stem}.toml"));

        let map_info_handle = load_context.load(map_info_path);

        let map_thumbnail_handle = if map_thumbnail_path.exists() {
            load_context.load(&*map_thumbnail_path)
        } else {
            load_context.load(MISSING_MAP_THUMBNAIL_PATH)
        };

        Ok(Self::Asset {
            map_thumbnail_handle,
            map_path,
            map_asset_handle: None,
            map_info_handle,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}

#[derive(Asset, Deserialize, Serialize, TypePath)]
pub struct SimpleWarfareMapInfo {
    pub title: String,
    pub description: String,
    pub author: String,
}

#[derive(Debug, Error)]
pub enum SimpleWarfareMapInfoLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse file: {0}")]
    Parse(#[from] toml::de::Error),
}

#[derive(Default)]
pub struct SimpleWarfareMapInfoLoader;

impl AssetLoader for SimpleWarfareMapInfoLoader {
    type Asset = SimpleWarfareMapInfo;

    type Settings = ();

    type Error = SimpleWarfareMapInfoLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut data = String::new();
        reader.read_to_string(&mut data).await?;
        Ok(toml::from_str(&data)?)
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
