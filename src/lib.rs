pub mod app_state;
pub mod assets;
pub mod bevy_ext;
pub mod js_engine;
pub mod lua_engine;
pub mod mod_engine;
pub mod panel;
pub mod scenes;
pub mod unit;
pub mod utils;
pub mod statistics;


use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_light_2d::prelude::*;
use bevy_panic_handler::PanicHandler;
use js_engine::JsEnginePlugin;

use crate::{
    app_state::AppState, assets::AssetsPlugin, lua_engine::LuaEnginePlugin,
    mod_engine::ModEnginePlugin, scenes::ScenePlugin, unit::UnitPlugin,
};

pub struct SimpleWarfarePlugins;

impl PluginGroup for SimpleWarfarePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(AssetsPlugin)
            .add(JsEnginePlugin)
            .add(LuaEnginePlugin)
            .add(ScenePlugin)
            .add(ModEnginePlugin)
            .add(UnitPlugin);
        group
    }
}

pub struct SimpleWarfarePlugin;

impl Plugin for SimpleWarfarePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .register_type::<AppState>()
            .add_plugins(StateInspectorPlugin::<AppState>::default())
            .add_plugins(FlyCameraPlugin)
            .add_plugins(PanicHandler::new().build())
            .add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::default())
            .add_plugins(Light2dPlugin)
            .add_plugins(SimpleWarfarePlugins);
    }
}
