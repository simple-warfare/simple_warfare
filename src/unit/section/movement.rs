use bevy::prelude::*;
use boa_engine::{JsResult, prelude::*, value::TryFromJs};
#[derive(Debug, Clone, Copy, Component, Reflect, TryFromJs)]
pub struct Movement {
    #[boa(rename = "movementType")]
    pub movement_type: MovementType,
    #[boa(rename = "moveSpeed")]
    pub move_speed: f64,
    #[boa(rename = "moveAccelerationSpeed")]
    pub move_acceleration_speed: f64,
    #[boa(rename = "moveDecelerationSpeed")]
    pub move_deceleration_speed: f64,
    #[boa(rename = "reverseSpeedPercentage")]
    pub reverse_speed_percentage: f64,
    #[boa(rename = "maxTurnSpeed")]
    pub max_turn_speed: f64,
    #[boa(rename = "turnAcceleration")]
    pub turn_acceleration: f64,
}
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub enum MovementType {
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
