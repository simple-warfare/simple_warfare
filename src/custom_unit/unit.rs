use bevy::prelude::*;

use crate::custom_unit::section::{collider::Colliders, core::Core, graphic::Graphics, movement::Movement};

#[derive(Debug, Component)]
pub struct CustomUnit;

#[derive(Debug, Clone)]
pub struct SpawnedUnitData {
    pub core: Core,
    pub graphics: Graphics,
    pub movement: Movement,
    pub colliders: Colliders,
}

impl SpawnedUnitData {
    pub fn new(core: Core, graphics: Graphics, movement: Movement, colliders: Colliders) -> Self {
        Self {
            core,
            graphics,
            movement,
            colliders,
        }
    }
}
