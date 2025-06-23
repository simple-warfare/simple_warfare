pub mod mods;
use bevy::prelude::*;

use crate::assets::mods::{info::*, lua::*};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ModInfo>()
            .init_asset_loader::<ModInfoLoader>()
            .init_asset::<LuaAsset>()
            .init_asset_loader::<LuaAssetLoader>();
    }
}
