use bevy::prelude::*;

use crate::custom_unit::section::Section;
#[derive(Debug, Component)]
pub struct Custom;

#[derive(Debug, Component)]
pub struct CustomUnit;

#[derive(Debug, Clone)]
pub struct SpawnedUnitData {
    pub section: Section,
}

impl SpawnedUnitData {
    pub fn new(section: Section) -> Self {
        Self { section }
    }
}
