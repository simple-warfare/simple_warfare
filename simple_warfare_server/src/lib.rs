pub mod adaptor;
pub mod assets;
pub mod bevy_ext;
pub mod cli;
pub mod consts;
pub mod custom;
pub mod debug;
pub mod helpers;
pub mod js_engine;
pub mod lightyear;
pub mod lua_engine;
pub mod mod_engine;
pub mod shared;
pub mod spatial;
pub mod states;
pub mod statistics;
pub mod system;
pub mod utils;

use bevy::{app::PluginGroupBuilder, prelude::*};
use js_engine::JsEnginePlugin;

use self::{
    adaptor::AdaptorServerPlugin,
    assets::AssetsPlugin,
    custom::{CustomPlugin, unit::CustomUnitPlugin},
    debug::DebugPlugin,
    js_engine::synchronize::SynchronizePlugin,
    lightyear::LightyearPlugin,
    lua_engine::LuaEnginePlugin,
    mod_engine::ModEnginePlugin,
    shared::SharedCustomPlugin,
    spatial::SpatialPlugin,
    states::ServerStatePlugin,
    statistics::StatistcsPlugin,
    system::SystemPlugin,
};

pub struct SimpleWarfareServerPlugins;

impl PluginGroup for SimpleWarfareServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(AssetsPlugin)
            .add(JsEnginePlugin)
            .add(LuaEnginePlugin)
            .add(ModEnginePlugin)
            .add(SystemPlugin)
            .add(StatistcsPlugin)
            .add(CustomUnitPlugin)
            .add(SpatialPlugin)
            .add(DebugPlugin)
            .add(SynchronizePlugin)
            .add(CustomPlugin)
            .add(SharedCustomPlugin)
            .add(ServerStatePlugin)
            .add(AdaptorServerPlugin)
            .add(LightyearPlugin);
        group
    }
}
