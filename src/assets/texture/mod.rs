use bevy::prelude::*;

use crate::{app_state::AppState, assets::{texture::interface::DialogTextureSlicer, GameAsset}};
pub mod interface;
pub struct TexturePlugin;

#[derive(Default, Resource)]
pub struct TextureAtlasLayoutHandles {
    pub dialog: Handle<TextureAtlasLayout>,
}

pub fn process_textures(
    mut system_state: ResMut<NextState<AppState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_atlas_layout_handles: ResMut<TextureAtlasLayoutHandles>,
    game_asset: Res<GameAsset>,
    images: ResMut<Assets<Image>>,
    mut dialog_texture_slicer: ResMut<DialogTextureSlicer>,
) {
    interface::process(
        &game_asset,
        &mut texture_atlases,
        &mut texture_atlas_layout_handles,
        &images,
        &mut dialog_texture_slicer,
    );
    system_state.set(AppState::LibsLoading);
}
