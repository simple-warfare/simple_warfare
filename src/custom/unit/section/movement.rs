use crate::bevy_ext::{try_from_js::*, try_into_js::*};
use avian2d::math::Scalar;
use bevy::{ecs::entity, prelude::*};
use boa_engine::{
    JsResult, js_string,
    prelude::*,
    value::{TryFromJs, TryIntoJs},
};
use serde::{Deserialize, Serialize};
use simple_warfare_macros::TryFromAndIntoJs;
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Copy, Component, TryFromAndIntoJs, Reflect,
)]
pub struct Movement {
    #[boa(
        from_js_with = "entity_try_from_js",
        into_js_with = "entity_try_into_js"
    )]
    pub entity: Entity,
    #[boa(rename = "movementType")]
    pub movement_type: MovementType,
    #[boa(
        from_js_with = "f32_try_from_js",
        into_js_with = "f32_try_into_js",
        rename = "maxMoveSpeed"
    )]
    pub max_move_speed: Scalar,
    #[boa(
        from_js_with = "f32_try_from_js",
        into_js_with = "f32_try_into_js",
        rename = "moveAcceleration"
    )]
    pub move_acceleration: Scalar,
    #[boa(
        from_js_with = "f32_try_from_js",
        into_js_with = "f32_try_into_js",
        rename = "moveDeceleration"
    )]
    pub move_deceleration: Scalar,
    #[boa(
        from_js_with = "f32_try_from_js",
        into_js_with = "f32_try_into_js",
        rename = "reversePercentage"
    )]
    pub reverse_percentage: Scalar,
    #[boa(
        from_js_with = "f32_try_from_js",
        into_js_with = "f32_try_into_js",
        rename = "maxTurnSpeed"
    )]
    pub max_turn_speed: Scalar,
    #[boa(
        from_js_with = "f32_try_from_js",
        into_js_with = "f32_try_into_js",
        rename = "turnAcceleration"
    )]
    pub turn_acceleration: Scalar,
    #[boa(
        from_js_with = "f32_try_from_js",
        into_js_with = "f32_try_into_js",
        rename = "turnDeceleration"
    )]
    pub turn_deceleration: Scalar,
}

impl Movement {
    pub fn new(
        entity: Entity,
        movement_type: MovementType,
        max_move_speed: Scalar,
        move_acceleration: Scalar,
        move_deceleration: Scalar,
        reverse_percentage: Scalar,
        max_turn_speed: Scalar,
        turn_acceleration: Scalar,
        turn_deceleration: Scalar,
    ) -> Self {
        Self {
            entity,
            movement_type,
            max_move_speed,
            move_acceleration,
            move_deceleration,
            reverse_percentage,
            max_turn_speed,
            turn_acceleration,
            turn_deceleration,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Copy, Component, Reflect)]
pub enum MovementType {
    #[default]
    Land,
}

impl TryFromJs for MovementType {
    fn try_from_js(value: &JsValue, _context: &mut Context) -> JsResult<Self> {
        match value {
            JsValue::String(movement_type) => match movement_type.to_std_string_lossy().as_str() {
                "Land" => Ok(MovementType::Land),
                _ => Err(JsNativeError::typ()
                    .with_message("cannot convert value to a movement_type")
                    .into()),
            },
            _ => Err(JsNativeError::typ()
                .with_message("cannot convert value to a movement_type")
                .into()),
        }
    }
}

impl TryIntoJs for MovementType {
    fn try_into_js(&self, _context: &mut Context) -> JsResult<JsValue> {
        match self {
            MovementType::Land => Ok(JsValue::String(js_string!("Land"))),
        }
    }
}
