pub mod app_state;
pub mod assets;
pub mod bevy_ext;
pub mod mod_engine;
pub mod unit;
pub mod utils;
pub mod js_engine;

use bevy::{app::PluginGroupBuilder, prelude::*};
use js_engine::SmilodonEnginePlugin;

use crate::{app_state::AppState, assets::AssetsPlugin, mod_engine::ModEnginePlugin};

pub struct SimpleWarfarePlugins;

impl PluginGroup for SimpleWarfarePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(AssetsPlugin)
            .add(SmilodonEnginePlugin)
            .add(ModEnginePlugin);
        group
    }
}

pub struct SimpleWarfarePlugin;

impl Plugin for SimpleWarfarePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(SimpleWarfarePlugins);
    }
}
