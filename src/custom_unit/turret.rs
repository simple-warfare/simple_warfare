use crate::bevy_ext::try_from_js::*;
use crate::custom_unit::{section::graphic::Graphic, transform::transform::JsTransform};
use crate::js_engine::global::class::entity::JsEntity;
use avian2d::math::Scalar;
use bevy::prelude::*;
use boa_engine::value::TryFromJs;

#[derive(Debug, Clone, Component, Reflect, TryFromJs)]

pub struct JsTurret {
    #[boa(from_js_with = "entity_try_from_js")]
    pub entity: Entity,
    pub transform: JsTransform,
    pub image: Graphic,
    #[boa(from_js_with = "f32_try_from_js", rename = "turnSpeed")]
    pub turn_speed: f32,
    #[boa(rename = "canShoot")]
    pub can_shoot: bool,
    #[boa(from_js_with = "f32_try_from_js", rename = "attackRadius")]
    pub attack_radius: Scalar,
    #[boa(rename = "UnitsInRange")]
    pub units_in_range: Vec<JsEntity>,
    #[boa(from_js_with = "entity_try_from_js", rename = "OnUnitEnterSignalEntity")]
    pub on_unit_enter_signal_entity: Entity,
    #[boa(from_js_with = "entity_try_from_js", rename = "OnUnitExitSignalEntity")]
    pub on_unit_exit_signal_entity: Entity,
}

#[derive(Debug, Default, Clone, Component, Reflect, TryFromJs)]
pub struct Turrets {
    pub data: Vec<JsTurret>,
}

impl Turrets {
    pub fn new(turrets: Vec<JsTurret>) -> Self {
        Self { data: turrets }
    }
}
