use bevy::prelude::*;

use crate::{
    assets::mods::{
        info::{ModEnable, ModInfo},
        js::JsAsset,
    },
    unit::custom_unit::SpawnedUnitData,
};

#[derive(Event, Clone)]
pub enum JsEngineRequestEvent {
    LoadMod(ModEnable, ModInfo),
    SpawnUnit(Entity, String),
}

#[derive(Event, Clone)]
pub enum JsEngineResponseEvent {
    EngineInited,
    SpawnedUnit(Entity, String, SpawnedUnitData),
}
#[derive(Debug, Event, Clone)]
pub enum SwModuleLoaderRequestEvent {
    LoadJsAsset(String),
}
#[derive(Debug, Event, Clone)]
pub enum SwModuleLoaderResponseEvent {
    LoadedJsAsset(JsAsset),
}
#[derive(Debug, Event, Clone)]
pub enum SwRequireLoaderRequestEvent {
    LoadJsAsset(String),
}
#[derive(Debug, Event, Clone)]
pub enum SwRequireLoaderResponseEvent {
    LoadedJsAsset(JsAsset),
}
