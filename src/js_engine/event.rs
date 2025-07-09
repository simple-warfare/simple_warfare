use bevy::prelude::*;

use crate::{
    assets::mods::{
        info::{ModEnable, ModInfo},
        js::JsAsset,
    },
    custom_unit::unit::SpawnedUnitData,
    js_engine::{
        global::class::entity::JsEntity,
        sw::{LookType, TeleportType},
    },
};

#[derive(Event)]
pub enum JsEngineRequestEvent {
    LoadMod(ModEnable, ModInfo),
    SpawnUnit(String),
    InsertEntity(JsEntity, Entity),
    ToTeleport(TeleportType),
    ToLook(LookType),
    //Signal
    SelectedSignalEmit,

    OnUnitEnterSignal(Vec<JsEntity>, Entity),
    OnUnitExitSignal(Vec<JsEntity>, Entity),
    //RemoteJsProxy(Box<dyn Fn(JsProxy) -> String + Send + Sync + 'static>),
}

#[derive(Debug, Event, Clone)]
pub enum JsEngineResponseEvent {
    EngineInited,
    //Mod
    SpawnedUnit(Entity, String, SpawnedUnitData),
    EntityToTeleport(EntityTeleportType),
    EntityToLook(EntityLookType),
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
