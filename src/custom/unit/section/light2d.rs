use bevy::prelude::*;
use boa_engine::value::TryFromJs;

use crate::custom::unit::light2d::point_light2d::JsPointLight2d;

/// mod单位拥有一个Array来存储点光源,在rust中我们将使用Vec
#[derive(Debug, Default, Clone, Component, Reflect, TryFromJs)]
pub struct JsPointLights2d {
    pub data: Vec<JsPointLight2d>,
}

impl JsPointLights2d {
    pub fn new(js_point_light2d: Vec<JsPointLight2d>) -> Self {
        Self {
            data: js_point_light2d,
        }
    }
    /// 将所有点光源收集到一起
    pub fn to_point_light2d(&self) -> Vec<impl Bundle> {
        self.data
            .iter()
            .map(|js_point_light2d| js_point_light2d.to_point_light2d())
            .collect()
    }
}
