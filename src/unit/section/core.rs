use bevy::prelude::*;
use boa_engine::value::TryFromJs;
#[derive(Debug, Clone, Component, Reflect, TryFromJs)]
pub struct Core {
    pub name: String,
    pub hp: u32,
    pub price: u32,
    pub mass: u32,
    #[boa(rename = "buildSpeed")]
    pub build_peed: f64,
    pub radius: f64,
    #[boa(rename = "maxHp")]
    pub max_hp: u32,
}
