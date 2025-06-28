use bevy::prelude::*;

use crate::assets::mods::info::{ModEnable, ModInfo};

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
    LoadMod(ModEnable, ModInfo),
    SpawnUnit(String),
}
