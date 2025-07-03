use bevy::prelude::*;

use crate::unit::section::{core::Core, graphic::Graphics, movement::Movement};

#[derive(Debug, Component)]
pub struct CustomUnit;

#[derive(Debug, Clone)]
pub struct SpawnedUnitData {
    pub core: Core,
    pub graphics: Graphics,
    pub movement: Movement,
}

impl SpawnedUnitData {
    pub fn new(core: Core, graphics: Graphics, movement: Movement) -> Self {
        Self {
            core,
            graphics,
            movement,
        }
    }
}
