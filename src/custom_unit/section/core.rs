use crate::bevy_ext::try_from_js::*;
use bevy::prelude::*;
use boa_engine::value::TryFromJs;
#[derive(Debug, Default, Clone, Component, Reflect, TryFromJs)]
pub struct Core {
    pub name: String,
    pub hp: u32,
    pub price: u32,
    pub max_hp: u32,
    #[boa(from_js_with = "f32_try_from_js")]
    pub mass: f32,
    #[boa(from_js_with = "f32_try_from_js")]
    #[boa(rename = "buildSpeed")]
    pub build_speed: f32,
    #[boa(from_js_with = "f32_try_from_js")]
    pub radius: f32,
    #[boa(rename = "enablePhysics")]
    pub enable_physics: bool,
}

impl Core {
    pub fn new(
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
