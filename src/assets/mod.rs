pub mod mods;
pub mod assets;
use bevy::prelude::*;

use crate::assets::mods::{info::*, js::*, lua::*};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ModInfo>()
            .init_asset_loader::<ModInfoLoader>()
            .init_asset::<LuaAsset>()
            .init_asset_loader::<LuaAssetLoader>()
            .init_asset::<JsAsset>()
            .init_asset_loader::<JsAssetLoader>();
    }
}
