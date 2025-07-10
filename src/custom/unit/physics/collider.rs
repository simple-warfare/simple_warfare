use avian2d::{math::Scalar, prelude::Collider};
use bevy::prelude::*;
use boa_engine::{JsResult, js_string, prelude::*, value::TryFromJs};

#[derive(Debug, Clone, Component, Reflect)]
pub enum JsCollider {
    Circle(Scalar),
    Rectangle(Scalar, Scalar),
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

impl JsCollider {
    pub fn to_avian2d(&self) -> Collider {
        match self {
            JsCollider::Circle(radius) => Collider::circle(*radius),
            JsCollider::Rectangle(x_length, y_length) => Collider::rectangle(*x_length, *y_length),
        }
    }
}
