use bevy::prelude::*;
use boa_engine::prelude::*;
use boa_engine::{JsResult, value::TryFromJs};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Hash, Component, Reflect, Serialize, Deserialize, PartialEq, Eq)]
pub enum JsSignalType {
    Custom,
    Created,
    Selected,
    OnUnitEnter,
    OnUnitExit,
    NewWayPoint,
    ActiveWayPointChanged,
    FixedUpdate,
}

impl TryFromJs for JsSignalType {
    fn try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Self> {
        match value.to_string(context)?.to_std_string_lossy().as_str() {
            "Custom" => Ok(Self::Custom),
            "Created" => Ok(Self::Created),
            "Selected" => Ok(Self::Selected),
            "OnUnitEnter" => Ok(Self::OnUnitEnter),
            "OnUnitExit" => Ok(Self::OnUnitExit),
            "NewWayPoint" => Ok(Self::NewWayPoint),
            "FixedUpdate" => Ok(Self::FixedUpdate),
            "ActiveWayPointChanged" => Ok(Self::ActiveWayPointChanged),
            _ => Err(JsNativeError::typ()
                .with_message("the DefaultSignal type is undefine")
                .into()),
        }
    }
}
