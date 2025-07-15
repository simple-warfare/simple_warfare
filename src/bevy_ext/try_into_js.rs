use bevy::prelude::*;
use boa_engine::{
    JsResult,
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
