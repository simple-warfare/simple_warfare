use bevy::prelude::*;
use boa_engine::{JsResult, js_string, prelude::*, value::TryFromJs};

use crate::statistics::Avian2dCollider;

#[derive(Debug, Clone, Component, Reflect)]
pub enum Collider {
    Circle(f32),
    Rectangle(f32, f32),
}

impl TryFromJs for Collider {
    fn try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Self> {
        let collider_object = value.to_object(context)?;
        match collider_object
            .get(js_string!("type"), context)?
            .to_string(context)?
            .to_std_string_lossy()
            .as_str()
        {
            "Circle" => Ok(Self::Circle(
                collider_object
                    .get(js_string!("radius"), context)?
                    .to_f32(context)?,
            )),
            "Rectangle" => Ok(Self::Rectangle(
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

impl Collider {
    pub fn to_avian2d(&self) -> Avian2dCollider {
        match self {
            Collider::Circle(radius) => {
                Avian2dCollider::circle(*radius)
            }
            Collider::Rectangle(x_length, y_length) => {
                Avian2dCollider::rectangle(*x_length, *y_length)
            }
        }
    }
}
