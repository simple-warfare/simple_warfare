use bevy::prelude::*;

use crate::assets::{GameAsset, texture::TextureAtlasLayoutHandles};

#[derive(Debug, PartialEq, Eq)]
pub enum DialogAtlasKind {
    MainMenu = 0,
    GrayButton = 1,
}

#[derive(Default, Resource)]
pub struct DialogTextureSlicer {
    pub main_menu: TextureSlicer,
    pub gray_button: TextureSlicer,
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

        texture_atlas.add_texture(URect::from_corners(
            UVec2::new(511, 463),
            UVec2::new(721, 497),
        ));
        texture_atlas.add_texture(URect::from_corners(
            UVec2::new(511, 0),
            UVec2::new(639, 127),
        ));

        dialog_texture_slicer.main_menu = TextureSlicer {
            border: BorderRect::axes(190., 15.),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 4.0,
        };

        dialog_texture_slicer.gray_button = TextureSlicer {
            border: BorderRect::axes(122., 122.),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 10.0,
        };
        texture_atlas_layout_handles.dialog = texture_atlases.add(texture_atlas);
    }
}
