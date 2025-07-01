use bevy::prelude::*;

use crate::assets::mods::info::{ModEnable, ModInfo};

#[derive(Event, Clone)]
pub enum JsEngineRequestEvent {
    ModEvent(ModEvent),
}

#[derive(Event, Clone)]
pub enum JsEngineResponeEvent {
    EngineInited,
}

#[derive(Clone)]
pub enum BuilderEvent {}

#[derive(Clone)]
pub enum ModEvent {
    LoadMod(ModEnable, ModInfo),
    SpawnUnit(Entity, String),
}
