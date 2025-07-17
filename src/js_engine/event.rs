use bevy::prelude::*;

use crate::{
    assets::mods::{
        info::ModInfo,
        js::JsAsset,
    },
    custom::unit::{section::core::Core, unit::SpawnedUnitData},
    js_engine::{
        global::class::entity::JsEntity,
        sw::{LookType, TeleportType}, synchronize::SynchronizeData,
    }, lua_engine::user_data::ModEnableClasses
};

#[derive(Event)]
pub enum JsEngineRequestEvent {
    LoadMod(ModEnableClasses, ModInfo),
    SpawnUnit(String),
    InsertEntity(JsEntity, Entity),
    ToTeleport(TeleportType),
    ToLook(LookType),
    //Signal
    SelectedSignalEmit,

    OnUnitEnterSignal(Vec<JsEntity>, Entity),
    OnUnitExitSignal(Vec<JsEntity>, Entity),
    EmitEmptySignal(JsEntity),
    SynchronizeData(SynchronizeData)
    //RemoteJsProxy(Box<dyn Fn(JsProxy) -> String + Send + Sync + 'static>),
}

#[derive(Debug, Event, Clone)]
pub enum JsEngineResponseEvent {
    EngineInited,
    //Mod
    SpawnedUnit(Entity, String, SpawnedUnitData),
    EntityToTeleport(EntityTeleportType),
    EntityToLook(EntityLookType),

    SynchronizeCore(Core)
}

#[derive(Debug, Clone, Copy)]
pub enum EntityTeleportType {
    Position(Entity, Vec2),
    Entity(Entity, Entity),
}

#[derive(Debug, Clone, Copy)]
pub enum EntityLookType {
    Position(Entity, Vec2),
    Entity(Entity, Entity),
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
