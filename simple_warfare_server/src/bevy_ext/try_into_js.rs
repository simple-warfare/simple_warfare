use bevy::prelude::*;
use boa_engine::{
    JsResult,
    object::builtins::{JsFloat32Array, JsTypedArray},
    prelude::*,
    value::TryIntoJs,
};

use crate::js_engine::global::class::entity::JsEntity;
pub fn f32_try_into_js(value: &f32, _context: &mut Context) -> JsResult<JsValue> {
    Ok(JsValue::Rational(*value as f64))
}

pub fn entity_try_into_js(value: &Entity, context: &mut Context) -> JsResult<JsValue> {
    JsEntity::from_entity(value).try_into_js(context)
}

pub fn vec2_try_into_js(value: &Vec2, context: &mut Context) -> JsResult<JsValue> {
    let vec2_array = JsFloat32Array::from_iter(vec![value.x, value.y], context)?;
    Ok(JsValue::Object(vec2_array.into()))
}

pub fn vec3_try_into_js(value: &Vec3, context: &mut Context) -> JsResult<JsValue> {
    let vec3_array = JsFloat32Array::from_iter(vec![value.x, value.y, value.z], context)?;
    Ok(JsValue::Object(vec3_array.into()))
}

pub fn option_entity_try_into_js(
    value: &Option<Entity>,
    context: &mut Context,
) -> JsResult<JsValue> {
    if let Some(entity) = value {
        entity_try_into_js(entity, context)
    } else {
        JsResult::Ok(JsValue::Undefined)
    }
}

pub fn quat_try_into_js(value: &Quat, context: &mut Context) -> JsResult<JsValue> {
    let quat_xyzw = value.to_array();

    JsResult::Ok(JsValue::Object(
        JsFloat32Array::from_iter(quat_xyzw, context)?.into(),
    ))
}
