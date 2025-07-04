use bevy::prelude::*;

use crate::{
    assets::mods::{
        info::{ModEnable, ModInfo},
        js::JsAsset,
    },
    js_engine::global::class::entity::JsEntity,
    unit::custom_unit::SpawnedUnitData,
};

#[derive(Debug, Event, Clone)]
pub enum JsEngineRequestEvent {
    LoadMod(ModEnable, ModInfo),
    SpawnUnit(Entity, String),
    GetEntityToTeleport(JsEntity, Vec2),
}

#[derive(Debug, Event, Clone)]
pub enum JsEngineResponseEvent {
    EngineInited,
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
