use bevy::prelude::*;

use crate::assets::mods::{
    info::{ModEnable, ModInfo},
    js::JsAsset,
};

#[derive(Event, Clone)]
pub enum JsEngineRequestEvent {
    LoadMod(ModEnable, ModInfo),
    SpawnUnit(Entity, String),
}

#[derive(Event, Clone, PartialEq, Eq)]
pub enum JsEngineResponseEvent {
    EngineInited,
    SpawnedUnit(Entity),
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
