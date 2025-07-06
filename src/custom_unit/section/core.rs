use bevy::prelude::*;
use boa_engine::{Context, JsResult, JsValue, js_string, value::TryFromJs};
#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct Core {
    pub name: String,
    pub hp: u32,
    pub price: u32,
    pub mass: f32,
    pub build_speed: f32,
    pub radius: f32,
    pub max_hp: u32,
    pub enable_physics: bool,
}

impl Core {
    pub fn new(
        name: String,
        hp: u32,
        price: u32,
        mass: f32,
        build_speed: f32,
        radius: f32,
        max_hp: u32,
        enable_physics: bool,
    ) -> Self {
        Self {
            name,
            hp,
            price,
            mass,
            build_speed,
            radius,
            max_hp,
            enable_physics,
        }
    }
}

impl TryFromJs for Core {
    fn try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Self> {
        let object = value.to_object(context)?;
        Ok(Self::new(
            object
                .get(js_string!("name"), context)?
                .to_string(context)?
                .to_std_string_lossy(),
            object.get(js_string!("hp"), context)?.to_u32(context)?,
            object.get(js_string!("price"), context)?.to_u32(context)?,
            object.get(js_string!("mass"), context)?.to_f32(context)?,
            object
                .get(js_string!("buildSpeed"), context)?
                .to_f32(context)?,
            object.get(js_string!("radius"), context)?.to_f32(context)?,
            object.get(js_string!("maxHp"), context)?.to_u32(context)?,
            object
                .get(js_string!("enable_physics"), context)?
                .to_boolean(),
        ))
    }
}
