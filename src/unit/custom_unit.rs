use bevy::prelude::*;

use crate::unit::section::{
    core::Core,
    graphic::{Graphic, Graphics},
};

#[derive(Component)]
pub struct CustomUnit;

#[derive(Debug, Clone)]
pub struct SpawnedUnitData {
    pub core: Core,
    pub graphics: Graphics,
}

impl SpawnedUnitData {
    pub fn new(core: Core, graphics: Graphics) -> Self {
        Self { core, graphics }
    }
}
