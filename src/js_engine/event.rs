use bevy::prelude::*;

use crate::assets::mods::js::JsAsset;

#[derive(Event, Clone)]
pub enum JsEngineEvent {
    EngineInited,
    BuilderEvent(BuilderEvent),
    ModEvent(ModEvent),
}

#[derive(Clone)]
pub enum BuilderEvent {}

#[derive(Clone)]
pub enum ModEvent {
    LoadJs(JsAsset),
    EnableUnit(String),
}
