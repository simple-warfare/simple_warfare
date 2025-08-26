use std::collections::VecDeque;

use bevy::prelude::*;
use boa_engine::{
    JsResult, js_string,
    object::ObjectInitializer,
    prelude::*,
    property::Attribute,
    value::TryIntoJs,
};

use crate::bevy_ext::try_into_js::vec2_try_into_js;

#[derive(Debug, Clone, Reflect, Copy)]
pub enum WayPoint {
    Move(Vec2),
    Attack(),
}

impl WayPoint {
    pub const TYPE_KEY: &'static str = "type";
    pub const MOVE_TYPE: &'static str = "move";
    pub const MOVE_POSIYION_KEY: &'static str = "position";
}

impl TryIntoJs for WayPoint {
    fn try_into_js(&self, context: &mut Context) -> JsResult<JsValue> {
        let way_point = match self {
            WayPoint::Move(vec2) => {
                let position = vec2_try_into_js(vec2, context)?;
                ObjectInitializer::new(context)
                    .property(
                        js_string!(Self::TYPE_KEY),
                        js_string!(Self::MOVE_TYPE),
                        Attribute::READONLY,
                    )
                    .property(
                        js_string!(Self::MOVE_POSIYION_KEY),
                        position,
                        Attribute::READONLY,
                    )
                    .build()
            }
            WayPoint::Attack() => todo!(),
        };

        JsResult::Ok(JsValue::Object(way_point.into()))
    }
}

#[derive(Debug, Default, Component, Reflect, Clone)]
pub struct WayPointQueue {
    pub data: VecDeque<WayPoint>,
}
