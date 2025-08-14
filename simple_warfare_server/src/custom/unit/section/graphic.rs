use crate::{bevy_ext::prelude::*, custom::unit::animation::trick_film_player::TrickFilmPlayer};
use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
    sprite::Anchor,
};
use boa_engine::value::TryFromJs;
use serde::{Deserialize, Serialize};
use simple_warfare_macros::TryFromAndIntoJs;

#[derive(Debug, Clone, Component, Serialize, Deserialize, PartialEq, Reflect, TryFromJs)]
pub struct Graphic {
    #[boa(from_js_with = "entity_try_from_js")]
    pub entity: Entity,
    pub width: u32,
    pub height: u32,
    pub path: String,
    pub layer: u32,
    #[boa(rename = "frameWidth")]
    pub frame_width: Option<u32>,
    #[boa(rename = "frameHeight")]
    pub frame_height: Option<u32>,

    #[boa(rename = "lockRotation", from_js_with = "option_f32_try_from_js")]
    pub lock_rotation: Option<f32>,
    #[boa(
        rename = "textureAtlasLayout",
        from_js_with = "texture_atlas_layout_try_from_js"
    )]
    pub texture_atlas_layout: Option<TextureAtlasLayout>,
    #[boa(from_js_with = "vec2_try_from_js")]
    pub offset: Vec2,
    #[boa(rename = "realPath")]
    pub real_path: Option<String>,

    #[boa(rename = "realParentPath")]
    pub real_parent_path: Option<String>,
    #[boa(rename = "trickFilmPlayer")]
    pub trick_film_player: Option<TrickFilmPlayer>,
}

impl Graphic {
    /// 将mod贴图定义的x,y偏移值转化成Bevy Sprite的锚点
    pub fn anchor(&self) -> Anchor {
        if let (Some(frame_width), Some(frame_height)) = (self.frame_width, self.frame_height) {
            Anchor::Custom(Vec2::new(
                self.offset.x / frame_width as f32,
                self.offset.y / frame_height as f32,
            ))
        } else {
            Anchor::Custom(Vec2::new(
                self.offset.x / self.width as f32,
                self.offset.y / self.height as f32,
            ))
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Component, Reflect, TryFromJs)]
pub struct Graphics {
    pub data: Vec<Graphic>,
}

impl Graphics {
    pub fn new(graphics: Vec<Graphic>) -> Self {
        Self { data: graphics }
    }
}
