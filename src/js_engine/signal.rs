use bevy::prelude::*;
use boa_engine::prelude::*;
use boa_engine::{JsResult, value::TryFromJs};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Hash, Component, Reflect, Serialize, Deserialize, PartialEq, Eq)]
pub enum JsDefaultSignalType {
    Created,
    Selected,
    OnUnitEnter,
    OnUnitExit,
    NewWayPoint,
    ActiveWayPointChanged,
}

impl TryFromJs for JsDefaultSignalType {
    fn try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Self> {
        match value.to_string(context)?.to_std_string_lossy().as_str() {
            "Created" => Ok(Self::Created),
            "Selected" => Ok(Self::Selected),
            "OnUnitEnter" => Ok(Self::OnUnitEnter),
            "OnUnitExit" => Ok(Self::OnUnitExit),
            "NewWayPoint" => Ok(Self::NewWayPoint),
            "ActiveWayPointChanged" => Ok(Self::ActiveWayPointChanged),
            _ => Err(JsNativeError::typ()
                .with_message("the DefaultSignal type is undefine")
                .into()),
        }
    }
}
