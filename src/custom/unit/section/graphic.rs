use crate::custom::unit::transform::transform::JsTransform;
use bevy::prelude::*;
use boa_engine::value::TryFromJs;

#[derive(Debug, Clone, Component, Reflect, TryFromJs)]
pub struct Graphic {
    pub transform: JsTransform,
    pub path: String,
    pub layer: u32,
    #[boa(rename = "frameWidth")]
    pub frame_width: Option<u32>,
    #[boa(rename = "frameHeight")]
    pub frame_height: Option<u32>,
}

#[derive(Debug, Default, Clone, Component, Reflect, TryFromJs)]
pub struct Graphics {
    pub data: Vec<Graphic>,
}

impl Graphics {
    pub fn new(graphics: Vec<Graphic>) -> Self {
        Self { data: graphics }
    }
}
