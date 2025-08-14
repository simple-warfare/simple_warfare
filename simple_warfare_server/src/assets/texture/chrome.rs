use bevy::prelude::*;

use crate::assets::{GameAsset, texture::TextureAtlasLayoutHandles};

#[derive(Debug, PartialEq, Eq)]
pub enum ChromeAtlasKind {
    MainViewer = 0,
    GrayBrickRect = 1,
}

#[derive(Default, Resource)]
pub struct ChromeTextureSlicer {
    pub main_viewer: TextureSlicer,
    pub gray_brick_rect: TextureSlicer,
}

pub(super) fn process(
    game_asset: &Res<GameAsset>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    texture_atlas_layout_handles: &mut ResMut<TextureAtlasLayoutHandles>,
    images: &ResMut<Assets<Image>>,
    chrome_texture_slicer: &mut ResMut<ChromeTextureSlicer>,
) {
    if let Some(chrome) = images.get(game_asset.interface.chrome.id()) {
        let mut texture_atlas = TextureAtlasLayout::new_empty(chrome.size());

        texture_atlas.add_texture(URect::from_corners(
            UVec2::new(0, 123),
            UVec2::new(434, 167),
        ));

        texture_atlas.add_texture(URect::from_corners(UVec2::new(56, 0), UVec2::new(84, 28)));

        chrome_texture_slicer.main_viewer = TextureSlicer {
            border: BorderRect::axes(190., 15.),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };
        chrome_texture_slicer.gray_brick_rect = TextureSlicer {
            border: BorderRect::all(22.),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };
        texture_atlas_layout_handles.chrome = texture_atlases.add(texture_atlas);
    }
}
