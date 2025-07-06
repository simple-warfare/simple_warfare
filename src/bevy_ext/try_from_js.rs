use bevy::prelude::*;
use boa_engine::{JsResult, object::builtins::JsTypedArray, prelude::*};

pub fn vec2_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Vec2> {
    let vec2_array = JsTypedArray::from_object(value.to_object(context)?)?;
    Ok(Vec2::new(
        vec2_array.at(0, context)?.to_f32(context)?,
        vec2_array.at(1, context)?.to_f32(context)?,
    ))
}

pub fn vec3_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Vec3> {
    let vec3_array = JsTypedArray::from_object(value.to_object(context)?)?;
    Ok(Vec3::new(
        vec3_array.at(0, context)?.to_f32(context)?,
        vec3_array.at(1, context)?.to_f32(context)?,
        vec3_array.at(2, context)?.to_f32(context)?,
    ))
}

pub fn quat_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Quat> {
    let quat_array = JsTypedArray::from_object(value.to_object(context)?)?;
    Ok(Quat::from_xyzw(
        quat_array.at(0, context)?.to_f32(context)?,
        quat_array.at(1, context)?.to_f32(context)?,
        quat_array.at(2, context)?.to_f32(context)?,
        quat_array.at(3, context)?.to_f32(context)?,
    ))
}
