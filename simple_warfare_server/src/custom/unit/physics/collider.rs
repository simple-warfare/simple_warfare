use bevy::prelude::*;
use boa_engine::{JsResult, js_string, prelude::*, value::TryFromJs};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Component, Reflect)]
pub enum JsCollider {
    Circle { radius: f32 },
    Rectangle { width: f32, hright: f32 },
}

impl TryFromJs for JsCollider {
    fn try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Self> {
        let collider_object = value.to_object(context)?;
        match collider_object
            .get(js_string!("type"), context)?
            .to_string(context)?
            .to_std_string_lossy()
            .as_str()
        {
            "Circle" => Ok(Self::circle(
                collider_object
                    .get(js_string!("radius"), context)?
                    .to_f32(context)?,
            )),
            "Rectangle" => Ok(Self::rectangle(
                collider_object
                    .get(js_string!("width"), context)?
                    .to_f32(context)?,
                collider_object
                    .get(js_string!("height"), context)?
                    .to_f32(context)?,
            )),
            _ => Err(JsNativeError::typ()
                .with_message("the collider type is undefine")
                .into()),
        }
    }
}

impl JsCollider {
    pub fn circle(radius: f32) -> Self {
        Self::Circle { radius }
    }
    pub fn rectangle(width: f32, hright: f32) -> Self {
        Self::Rectangle { width, hright }
    }
}
