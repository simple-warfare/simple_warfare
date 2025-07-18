use bevy::asset::Handle;

pub mod ldtk;
pub mod tiled;

#[derive(Debug, Clone)]
pub enum SimpleWarfareMap {
    Ldtk(Handle<ldtk::LdtkMap>),
    Tiled(Handle<tiled::TiledMap>),
}