pub mod js;
use bevy::prelude::*;
use js::*;
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<JsAsset>()
            .init_asset_loader::<JsAssetLoader>();
    }
}
