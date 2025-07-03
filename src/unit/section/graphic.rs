use crate::bevy_ext::try_from_js::try_from_js_to_vec2;
use bevy::prelude::*;
use boa_engine::value::TryFromJs;

#[derive(Debug, Clone, Component, Reflect, TryFromJs)]
pub struct Graphic {
    #[boa(from_js_with = "try_from_js_to_vec2")]
    pub position: Vec2,
    pub path: String,
    pub layer: u32,
    #[boa(rename = "frameWidth")]
    pub frame_width: Option<u32>,
    #[boa(rename = "frameHeight")]
    pub frame_height: Option<u32>,
}

#[derive(Debug, Clone, Component, Reflect, TryFromJs)]
pub struct Graphics {
    pub data: Vec<Graphic>,
}

impl Graphics {
    pub fn new(graphics: Vec<Graphic>) -> Self {
        Self { data: graphics }
    }
}
