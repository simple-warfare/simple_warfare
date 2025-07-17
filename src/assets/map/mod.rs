pub mod ldtk;
pub mod tiled;

#[derive(Debug, Clone)]
pub enum SimpleWarfareMap {
    Ldtk(ldtk::LdtkMap),
    Tiled(tiled::TiledMap),
}
