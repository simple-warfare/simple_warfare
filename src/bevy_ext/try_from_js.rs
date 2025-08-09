use bevy::prelude::*;
use boa_engine::{
    JsResult, js_string,
    object::builtins::{JsArray, JsTypedArray},
    prelude::*,
    value::TryFromJs,
};

use crate::js_engine::global::class::entity::JsEntity;

pub fn vec2_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Vec2> {
    let vec2_array = JsTypedArray::from_object(value.to_object(context)?)?;
    Ok(Vec2::new(
        vec2_array.at(0, context)?.to_f32(context)?,
        vec2_array.at(1, context)?.to_f32(context)?,
    ))
}
pub fn uvec2_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<UVec2> {
    let uvec2_array = JsTypedArray::from_object(value.to_object(context)?)?;
    Ok(UVec2::new(
        uvec2_array.at(0, context)?.to_u32(context)?,
        uvec2_array.at(1, context)?.to_u32(context)?,
    ))
}

pub fn urect2_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<URect> {
    let rectangle_object = value.to_object(context)?;
    let min = rectangle_object.get(js_string!("min"), context)?;
    let max = rectangle_object.get(js_string!("max"), context)?;
    Ok(URect {
        min: uvec2_try_from_js(&min, context)?,
        max: uvec2_try_from_js(&max, context)?,
    })
}

pub fn vec3_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Vec3> {
    let vec3_array = JsTypedArray::from_object(value.to_object(context)?)?;
    Ok(Vec3::new(
        vec3_array.at(0, context)?.to_f32(context)?,
        vec3_array.at(1, context)?.to_f32(context)?,
        vec3_array.at(2, context)?.to_f32(context)?,
    ))
}

pub fn f32_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<f32> {
    value.to_f32(context)
}

pub fn option_f32_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Option<f32>> {
    if value.is_undefined() {
        JsResult::Ok(None)
    } else {
        value.to_f32(context).map(|n| Some(n))
    }
}

pub fn entity_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Entity> {
    let js_entity = JsEntity::try_from_js(value, context).expect("try_from_js error");
    Ok(js_entity.to_entity())
}

pub fn option_entity_try_from_js(
    value: &JsValue,
    context: &mut Context,
) -> JsResult<Option<Entity>> {
    if value.is_undefined() {
        JsResult::Ok(None)
    } else {
        let js_entity = JsEntity::try_from_js(value, context).expect("try_from_js error");
        Ok(Some(js_entity.to_entity()))
    }
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

pub fn color_try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Color> {
    let color_object = value.to_object(context)?;

    let color = JsArray::from_object(
        color_object
            .get(js_string!("color"), context)?
            .to_object(context)?,
    )?;
    let valpha = color_object
        .get(js_string!("valpha"), context)?
        .to_f32(context)?;

    let model = color_object
        .get(js_string!("model"), context)?
        .to_string(context)?
        .to_std_string_lossy();

    match model.as_str() {
        "rgb" => Ok(Color::LinearRgba(LinearRgba::new(
            color.at(0, context)?.to_f32(context)?,
            color.at(1, context)?.to_f32(context)?,
            color.at(2, context)?.to_f32(context)?,
            valpha,
        ))),
        _ => Ok(Color::default()),
    }
}

pub fn texture_atlas_layout_try_from_js(
    value: &JsValue,
    context: &mut Context,
) -> JsResult<Option<TextureAtlasLayout>> {
    if value.is_undefined() {
        return Ok(None);
    }
    let texture_atlas_layout_object = value.to_object(context)?;

    let size = uvec2_try_from_js(
        &texture_atlas_layout_object.get(js_string!("size"), context)?,
        context,
    )?;

    let textures_arrary = JsArray::from_object(
        texture_atlas_layout_object
            .get(js_string!("textures"), context)?
            .to_object(context)?,
    )?;

    let textures = array_collect_to_vec(&textures_arrary, context, urect2_try_from_js)?;
    Ok(textures.map(|textures| TextureAtlasLayout { size, textures }))
}

pub fn array_collect_to_vec<T>(
    array: &JsArray,
    context: &mut Context,
    f: fn(&JsValue, &mut Context) -> JsResult<T>,
) -> JsResult<Option<Vec<T>>> {
    (0..array.length(context)?)
        .map(|i| f(&array.at(i as i64, context)?, context))
        .collect::<Result<Vec<_>, _>>()
        .map(|v| if v.is_empty() { None } else { Some(v) })
}
