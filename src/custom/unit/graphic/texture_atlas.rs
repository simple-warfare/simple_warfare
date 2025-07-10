use bevy::prelude::*;

#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct JsTextureAtlasLayout {
    pub size: UVec2,
    pub textures: Vec<URect>,
}
