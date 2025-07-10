use bevy::prelude::*;
use boa_engine::value::TryFromJs;

use crate::custom::unit::light2d::point_light2d::JsPointLight2d;
#[derive(Debug, Default, Clone, Component, Reflect, TryFromJs)]
pub struct JsPointLights2d {
    pub data: Vec<JsPointLight2d>,
}

impl JsPointLights2d {
    pub fn new(js_point_light2d: Vec<JsPointLight2d>) -> Self {
        Self { data: js_point_light2d }
    }

    pub fn to_point_light2d(&self) -> Vec<impl Bundle> {
        self.data
            .iter()
            .map(|js_point_light2d| js_point_light2d.to_point_light2d())
            .collect()
    }
}
