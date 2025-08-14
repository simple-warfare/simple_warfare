use bevy::prelude::*;

use crate::assets::{GameAsset, texture::TextureAtlasLayoutHandles};

#[derive(Debug, PartialEq, Eq)]
pub enum DialogAtlasKind {
    MainMenu = 0,
    GrayRect = 1,
    LightBlueRect = 2,
    DarkGrayRect1 = 3,
    DarkGrayRect2 = 4,
}

#[derive(Default, Resource)]
pub struct DialogTextureSlicer {
    pub main_menu: TextureSlicer,
    pub gray_rect: TextureSlicer,
    pub light_blue_rect: TextureSlicer,
    pub dark_gray_rect: TextureSlicer,
}

pub(super) fn process(
    game_assets: &Res<GameAsset>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    texture_atlas_layout_handles: &mut ResMut<TextureAtlasLayoutHandles>,
    images: &ResMut<Assets<Image>>,
    dialog_texture_slicer: &mut ResMut<DialogTextureSlicer>,
) {
    if let Some(dialog) = images.get(game_assets.interface.dialog.id()) {
        let mut texture_atlas = TextureAtlasLayout::new_empty(dialog.size());
        //MainMenu
        texture_atlas.add_texture(URect::from_corners(
            UVec2::new(511, 463),
            UVec2::new(721, 497),
        ));
        //GrayRect
        texture_atlas.add_texture(URect::from_corners(
            UVec2::new(511, 0),
            UVec2::new(639, 127),
        ));
        //LightBlueRect
        texture_atlas.add_texture(URect::from_corners(
            UVec2::new(512, 387),
            UVec2::new(576, 451),
        ));
        //DarkRyatRect1
        texture_atlas.add_texture(URect::from_corners(
            UVec2::new(768, 128),
            UVec2::new(895, 255),
        ));
        //DarkRyatRect2
        texture_atlas.add_texture(URect::from_corners(
            UVec2::new(896, 0),
            UVec2::new(1024, 128),
        ));

        dialog_texture_slicer.main_menu = TextureSlicer {
            border: BorderRect::axes(190., 15.),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };
        dialog_texture_slicer.gray_rect = TextureSlicer {
            border: BorderRect::axes(122., 122.),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };
        dialog_texture_slicer.light_blue_rect = TextureSlicer {
            border: BorderRect::all(29.),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };
        dialog_texture_slicer.dark_gray_rect = TextureSlicer {
            border: BorderRect::all(29.),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };
        texture_atlas_layout_handles.dialog = texture_atlases.add(texture_atlas);
    }
}
