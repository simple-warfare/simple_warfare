use crate::bevy_ext::try_from_js::try_from_js_to_vec2;
use bevy::math::Vec2;
use boa_engine::value::TryFromJs;

#[derive(Debug, TryFromJs)]
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
