use avian2d::math::Scalar;
use bevy::prelude::*;
use boa_engine::{JsResult, js_string, prelude::*, value::TryFromJs};
use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Copy, Component, Reflect)]
pub struct Movement {
    pub movement_type: MovementType,
    pub max_move_speed: Scalar,
    pub move_acceleration: Scalar,
    pub move_deceleration: Scalar,
    pub reverse_percentage: Scalar,
    pub max_turn_speed: Scalar,
    pub turn_acceleration: Scalar,
    pub turn_deceleration: Scalar,
}

impl Movement {
    pub fn new(
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

impl TryFromJs for Movement {
    fn try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Self> {
        let object = value.to_object(context)?;
        Ok(Movement::new(
            MovementType::try_from_js(&object.get(js_string!("movementType"), context)?, context)?,
            object
                .get(js_string!("maxMoveSpeed"), context)?
                .to_f32(context)?,
            object
                .get(js_string!("moveAcceleration"), context)?
                .to_f32(context)?,
            object
                .get(js_string!("moveDeceleration"), context)?
                .to_f32(context)?,
            object
                .get(js_string!("reversePercentage"), context)?
                .to_f32(context)?,
            object
                .get(js_string!("maxTurnSpeed"), context)?
                .to_f32(context)?,
            object
                .get(js_string!("turnAcceleration"), context)?
                .to_f32(context)?,
            object
                .get(js_string!("turnDeceleration"), context)?
                .to_f32(context)?,
        ))
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
