pub mod assets;
pub mod bevy_ext;
pub mod consts;
pub mod custom;
pub mod debug;
pub mod js_engine;
pub mod lua_engine;
pub mod map;
pub mod mod_engine;
pub mod net;
pub mod panel;
pub mod scenes;
pub mod spatial;
pub mod statistics;
pub mod system;
pub mod utils;
pub mod helpers;

use crate::{
    assets::AssetsPlugin,
    custom::{ui::CustomUiPlugin, unit::CustomUnitPlugin},
    debug::DebugPlugin,
    js_engine::synchronize::SynchronizePlugin,
    lua_engine::LuaEnginePlugin,
    map::MapPlugin,
    mod_engine::ModEnginePlugin,
    net::NetPlugin,
    panel::PanelPlugin,
    scenes::{ScenePlugin, SceneState},
    spatial::SpatialPlugin,
    statistics::{AppState, StatistcsPlugin},
    system::SystemPlugin,
};
use avian2d::prelude::*;
use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
};
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tiled::TiledMapPlugin;
use bevy_fly_camera::FlyCameraPlugin;
use bevy_hui::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::{StateInspectorPlugin, WorldInspectorPlugin},
};
use bevy_light_2d::prelude::*;

//use bevy_panic_handler::PanicHandler;
use bevy_seedling::SeedlingPlugin;
use js_engine::JsEnginePlugin;

pub struct SimpleWarfarePlugins;

impl PluginGroup for SimpleWarfarePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(AssetsPlugin)
            .add(JsEnginePlugin)
            .add(LuaEnginePlugin)
            .add(ScenePlugin)
            .add(PanelPlugin)
            .add(ModEnginePlugin)
            .add(SystemPlugin)
            .add(StatistcsPlugin)
            .add(CustomUnitPlugin)
            .add(SpatialPlugin)
            .add(DebugPlugin)
            .add(CustomUiPlugin)
            .add(SynchronizePlugin)
            .add(NetPlugin)
            .add(MapPlugin);
        group
    }
}

pub struct SimpleWarfarePlugin;

impl Plugin for SimpleWarfarePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AppState>()
            .register_type::<SceneState>()
            .add_plugins((
                RemotePlugin::default(),
                RemoteHttpPlugin::default(),
                HuiPlugin,
            ))
            .add_plugins(PhysicsPlugins::default())
            .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(EguiPlugin {
                enable_multipass_for_primary_context: true,
            })
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(StateInspectorPlugin::<AppState>::default())
            .add_plugins(StateInspectorPlugin::<SceneState>::default())
            .add_plugins(FlyCameraPlugin)
            //.add_plugins(PanicHandler::new().build())
            .add_plugins(LdtkPlugin)
            .add_plugins(AsepriteUltraPlugin)
            .insert_resource(LevelSelection::default())
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .add_plugins(TiledMapPlugin::default())
            .add_plugins(Light2dPlugin)
            .add_plugins(SeedlingPlugin::default())
            .add_plugins(SimpleWarfarePlugins)
            .insert_resource(Gravity(Vec2::ZERO));
    }
}
