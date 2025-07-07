pub mod app_state;
pub mod assets;
pub mod bevy_ext;
pub mod custom_unit;
pub mod js_engine;
pub mod lua_engine;
pub mod mod_engine;
pub mod panel;
pub mod scenes;
pub mod statistics;
pub mod system;
pub mod utils;

use avian2d::prelude::*;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_light_2d::prelude::*;
use bevy_panic_handler::PanicHandler;
use js_engine::JsEnginePlugin;

use crate::{
    app_state::AppState, assets::AssetsPlugin, custom_unit::CustomUnitPlugin,
    lua_engine::LuaEnginePlugin, mod_engine::ModEnginePlugin, scenes::ScenePlugin,
    statistics::StatistcsPlugin, system::SystemPlugin,
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
            .add(SystemPlugin)
            .add(StatistcsPlugin)
            .add(CustomUnitPlugin);
        group
    }
}

pub struct SimpleWarfarePlugin;

impl Plugin for SimpleWarfarePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .register_type::<AppState>()
            .add_plugins(EnhancedInputPlugin)
            .add_plugins(PhysicsPlugins::default())
            .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(StateInspectorPlugin::<AppState>::default())
            .add_plugins(FlyCameraPlugin)
            .add_plugins(PanicHandler::new().build())
            .add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::default())
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .add_plugins(Light2dPlugin)
            .add_plugins(SimpleWarfarePlugins)
            .insert_resource(Gravity(Vec2::ZERO));
    }
}
