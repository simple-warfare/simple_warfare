use std::path::PathBuf;

use bevy::{
    asset::{Assets, Handle},
    ecs::system::Res,
    image::Image,
};

use crate::assets::map::{
    ldtk::LdtkMap,
    tiled::{TiledMap, TiledMapInfo},
};

pub mod ldtk;
pub mod tiled;

#[derive(Debug, Clone)]
pub enum SimpleWarfareMap {
    Ldtk(Handle<ldtk::LdtkMap>),
    Tiled(Handle<tiled::TiledMap>),
}

impl SimpleWarfareMap {
    pub fn get_thumbnail(
        &self,
        tiled_maps: &Res<Assets<TiledMap>>,
        ldtk_maps: &Res<Assets<LdtkMap>>,
    ) -> Handle<Image> {
        match self {
            SimpleWarfareMap::Ldtk(map_handle) => {
                ldtk_maps.get(map_handle.id()).unwrap().thumbnail.clone()
            }
            SimpleWarfareMap::Tiled(map_handle) => {
                tiled_maps.get(map_handle.id()).unwrap().thumbnail.clone()
            }
        }
    }

    pub fn get_title(
        &self,
        tiled_maps: &Res<Assets<TiledMap>>,
        ldtk_maps: &Res<Assets<LdtkMap>>,
        tiled_map_infos: &Res<Assets<TiledMapInfo>>,
    ) -> String {
        match self {
            SimpleWarfareMap::Ldtk(map_handle) => {
                todo!()
            }
            SimpleWarfareMap::Tiled(map_handle) => tiled_map_infos
                .get(tiled_maps.get(map_handle.id()).unwrap().info.id())
                .unwrap()
                .title
                .clone(),
        }
    }

    pub fn get_path(
        &self,
        tiled_maps: &Res<Assets<TiledMap>>,
        ldtk_maps: &Res<Assets<LdtkMap>>,
    ) -> PathBuf {
        match self {
            SimpleWarfareMap::Ldtk(map_handle) => {
                todo!()
            }
            SimpleWarfareMap::Tiled(map_handle) => {
                tiled_maps.get(map_handle.id()).unwrap().path.clone()
            }
        }
    }
}
