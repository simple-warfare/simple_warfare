use bevy::prelude::*;

use crate::{
    custom::{CustomTypedId, unit::section::Section},
    net::shared::UnitId,
};

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
    pub custom_typed_id: CustomTypedId,
}

impl SpawnedUnitData {
    pub fn new(
        section: Section,
        unit_id: UnitId,
        entity: Entity,
        module_path: String,
        custom_typed_id: CustomTypedId,
    ) -> Self {
        Self {
            section,
            unit_id,
            entity,
            module_path,
            custom_typed_id,
        }
    }
}
