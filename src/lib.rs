pub mod app_state;
pub mod assets;
pub mod bevy_ext;
pub mod js_engine;
pub mod mod_engine;
pub mod panel;
pub mod scenes;
pub mod unit;
pub mod utils;

use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_light_2d::prelude::*;
use js_engine::SmilodonEnginePlugin;

use crate::{
    app_state::AppState, assets::AssetsPlugin, mod_engine::ModEnginePlugin, scenes::ScenePlugin,
};

pub struct SimpleWarfarePlugins;

impl PluginGroup for SimpleWarfarePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(AssetsPlugin)
            .add(SmilodonEnginePlugin)
            .add(ModEnginePlugin)
            .add(ScenePlugin);
        group
    }
}

pub struct SimpleWarfarePlugin;

impl Plugin for SimpleWarfarePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(SimpleWarfarePlugins)
            .add_plugins(FlyCameraPlugin)
            .add_plugins(bevy_panic_handler::PanicHandler::new().build())
            .add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::default())
            .add_plugins(Light2dPlugin);
    }
}
