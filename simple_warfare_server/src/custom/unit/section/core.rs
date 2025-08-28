use crate::bevy_ext::{try_from_js::*, try_into_js::*};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use simple_warfare_server_macros::TryFromAndIntoJs;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Component, Reflect, TryFromAndIntoJs)]
pub struct Core {
    #[boa(
        from_js_with = "entity_try_from_js",
        into_js_with = "entity_try_into_js"
    )]
    pub entity: Entity,
    pub name: String,
    pub hp: u32,
    pub price: u32,
    #[boa(rename = "maxHp")]
    pub max_hp: u32,
    #[boa(from_js_with = "f32_try_from_js", into_js_with = "f32_try_into_js")]
    pub mass: f32,
    #[boa(
        from_js_with = "f32_try_from_js",
        into_js_with = "f32_try_into_js",
        rename = "buildSpeed"
    )]
    pub build_speed: f32,
    #[boa(from_js_with = "f32_try_from_js", into_js_with = "f32_try_into_js")]
    pub radius: f32,
    #[boa(rename = "enablePhysics")]
    pub enable_physics: bool,
}

impl Core {
    pub fn new(
        entity: Entity,
        name: String,
        hp: u32,
        price: u32,
        mass: f32,
        build_speed: f32,
        radius: f32,
        max_hp: u32,
        enable_physics: bool,
    ) -> Self {
        Self {
            entity,
            name,
            hp,
            price,
            mass,
            build_speed,
            radius,
            max_hp,
            enable_physics,
        }
    }
}
