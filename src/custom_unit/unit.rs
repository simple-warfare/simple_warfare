use bevy::prelude::*;

use crate::custom_unit::section::{
    collider::JsColliders, core::Core, graphic::Graphics, light2d::JsPointLights2d, movement::Movement
};

#[derive(Debug, Component)]
pub struct CustomUnit;

#[derive(Debug, Clone)]
pub struct SpawnedUnitData {
    pub core: Core,
    pub graphics: Graphics,
    pub movement: Movement,
    pub colliders: JsColliders,
    pub point_lights: JsPointLights2d,
}

impl SpawnedUnitData {
    pub fn new(
        core: Core,
        graphics: Graphics,
        movement: Movement,
        colliders: JsColliders,
        point_lights: JsPointLights2d,
    ) -> Self {
        Self {
            core,
            graphics,
            movement,
            colliders,
            point_lights,
        }
    }
}
