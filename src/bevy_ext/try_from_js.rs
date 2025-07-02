use bevy::prelude::*;
use boa_engine::{JsResult, object::builtins::JsTypedArray, prelude::*};

pub fn try_from_js_to_vec2(value: &JsValue, context: &mut Context) -> JsResult<Vec2> {
    match value {
        JsValue::Object(vec2_object) => {
            let vec2_arry = JsTypedArray::from_object(vec2_object.clone())?;
            let maybe_x = vec2_arry.at(0, context)?;
            let maybe_y = vec2_arry.at(1, context)?;
            if vec2_arry.length(context)? == 2 && maybe_x.is_double() && maybe_y.is_double() {
                Ok(Vec2::new(
                    maybe_x.to_f32(context)?,
                    maybe_y.to_f32(context)?,
                ))
            } else {
                Err(JsNativeError::typ()
                    .with_message("cannot convert value to an vec2")
                    .into())
            }
        }
        _ => Err(JsNativeError::typ()
            .with_message("cannot convert value to an vec2")
            .into()),
    }
}
