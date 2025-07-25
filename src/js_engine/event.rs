use bevy::prelude::*;

use crate::{
    assets::mods::js::JsAsset,
    custom::{
        CustomModAsset,
        unit::{section::core::Core, unit::SpawnedUnitData},
    },
    js_engine::{
        global::class::entity::JsEntity,
        sw::{LookType, TeleportType},
        synchronize::SynchronizeData,
    },
};

#[derive(Event)]
pub enum JsEngineRequestEvent {
    LoadMod(CustomModAsset),
    SpawnUnit(String),
    InsertEntity(JsEntity, Entity),
    ToTeleport(TeleportType),
    ToLook(LookType),
    //Signal
    SelectedSignalEmit,

    OnUnitEnterSignal(Vec<JsEntity>, Entity),
    OnUnitExitSignal(Vec<JsEntity>, Entity),
    EmitEmptySignal(JsEntity),
    SynchronizeData(SynchronizeData), //RemoteJsProxy(Box<dyn Fn(JsProxy) -> String + Send + Sync + 'static>),
}

#[derive(Debug, Event, Clone)]
pub enum JsEngineResponseEvent {
    EngineInited,
    //Mod
    SpawnedUnit(Entity, String, SpawnedUnitData),
    EntityToTeleport(EntityTeleportType),
    EntityToLook(EntityLookType),

    SynchronizeCore(Core),
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
#[derive(Debug)]
pub enum SwModuleLoaderRequestEvent {
    LoadJsAsset {
        path: String,
        sender: Box<oneshot::Sender<JsAsset>>,
    },
}

impl SwModuleLoaderRequestEvent {
    pub fn load_js_asset(path: String, sender: Box<oneshot::Sender<JsAsset>>) -> Self {
        Self::LoadJsAsset { path, sender }
    }
}
pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JsEngineRequestEvent>()
            .add_event::<JsEngineResponseEvent>();
    }
}
