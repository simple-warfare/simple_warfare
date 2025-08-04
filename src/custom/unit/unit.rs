use bevy::prelude::*;

use crate::{custom::unit::section::Section, net::shared::UnitId};

#[derive(Debug, Component)]
pub struct Custom;

#[derive(Debug, Component)]
pub struct CustomTurrrt;

#[derive(Debug, Component)]
pub struct CustomUnit;

#[derive(Debug, Clone)]
pub struct SpawnedUnitData {
    pub unit_id: UnitId,
    pub entity: Entity,
    pub module_path: String,
    pub section: Section,
}

impl SpawnedUnitData {
    pub fn new(section: Section, unit_id: UnitId, entity: Entity, module_path: String) -> Self {
        Self {
            section,
            unit_id,
            entity,
            module_path,
        }
    }
}
