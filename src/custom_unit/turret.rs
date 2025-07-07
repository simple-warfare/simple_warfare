use crate::bevy_ext::try_from_js::*;
use bevy::prelude::*;
use boa_engine::value::TryFromJs;

use crate::custom_unit::{section::graphic::Graphic, transform::transform::JsTransform};

#[derive(Debug, Clone, Component, Reflect, TryFromJs)]

pub struct Turret {
    pub transform: JsTransform,
    pub image: Graphic,
    #[boa(from_js_with = "f32_try_from_js", rename = "turnSpeed")]
    pub turn_speed: f32,
    #[boa(rename = "canShoot")]
    pub can_shoot: bool,
}

#[derive(Debug, Default, Clone, Component, Reflect, TryFromJs)]
pub struct Turrets {
    pub data: Vec<Turret>,
}

impl Turrets {
    pub fn new(turrets: Vec<Turret>) -> Self {
        Self { data: turrets }
    }
}
