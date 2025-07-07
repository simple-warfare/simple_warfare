use bevy::prelude::*;
use boa_engine::object::builtins::JsProxy;

use crate::{
    assets::mods::{
        info::{ModEnable, ModInfo},
        js::JsAsset,
    },
    custom_unit::unit::SpawnedUnitData,
    js_engine::global::class::entity::JsEntity,
};

#[derive(Event)]
pub enum JsEngineRequestEvent {
    LoadMod(ModEnable, ModInfo),
    SpawnUnit(Entity, String),
    GetEntityToTeleport(JsEntity, Vec2),

    //Signal
    SignalEmit,
    SignalConnect,
    SelectedSignalEmit,
    //RemoteJsProxy(Box<dyn Fn(JsProxy) -> String + Send + Sync + 'static>),
}

#[derive(Debug, Event, Clone)]
pub enum JsEngineResponseEvent {
    EngineInited,
    //Mod
    SpawnedUnit(Entity, String, SpawnedUnitData),
    GetedEntityToTeleport(JsEntity, Entity, Vec2),
}
#[derive(Debug, Clone)]
pub enum SwModuleLoaderRequestEvent {
    LoadJsAsset(String),
}
#[derive(Debug, Clone)]
pub enum SwModuleLoaderResponseEvent {
    LoadedJsAsset(JsAsset),
}
#[derive(Debug, Clone)]
pub enum SwRequireLoaderRequestEvent {
    LoadJsAsset(String),
}
#[derive(Debug, Clone)]
pub enum SwRequireLoaderResponseEvent {
    LoadedJsAsset(JsAsset),
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JsEngineRequestEvent>()
            .add_event::<JsEngineResponseEvent>();
    }
}
