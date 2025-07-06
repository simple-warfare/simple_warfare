use bevy::{ecs::bundle::Bundle, reflect::Reflect};

use crate::custom_unit::section::{
    collider::JsColliders, core::Core, graphic::Graphics, light2d::JsPointLights2d,
    movement::Movement,
};

pub mod collider;
pub mod core;
pub mod graphic;
pub mod light2d;
pub mod movement;

#[derive(Debug, Default, Bundle, Reflect)]
pub struct Section {
    pub core: Core,
    pub colliders: JsColliders,
    pub graphics: Graphics,
    pub movement: Movement,
    pub point_lights: JsPointLights2d,
}

impl Section {
    pub fn new(
        core: Core,
        colliders: JsColliders,
        graphics: Graphics,
        movement: Movement,
        point_lights: JsPointLights2d,
    ) -> Self {
        Self {
            core,
            colliders,
            graphics,
            movement,
            point_lights
        }
    }
}
