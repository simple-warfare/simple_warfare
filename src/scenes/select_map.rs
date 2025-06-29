use bevy::{color::palettes::css::*, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    assets::{
        GameAsset,
        map::ldtk::LdtkMap,
        texture::{
            TextureAtlasLayoutHandles,
            chrome::{ChromeAtlasKind, ChromeTextureSlicer},
            interface::{DialogAtlasKind, DialogTextureSlicer},
        },
    },
    bevy_ext::app::AppExt,
};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct SelectMapScene;

#[derive(Component)]
struct SelectMapSceneMark;

impl Scene for SelectMapScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<SelectMapSceneMark, _, _>(SceneState::SelectMapScene, setup);
    }
}

fn setup(
    mut commands: Commands,
    game_asset: Res<GameAsset>,
    texture_atlas_layout_handles: Res<TextureAtlasLayoutHandles>,
    dialog_texture_slicer: Res<DialogTextureSlicer>,
    chrome_texture_slicer: Res<ChromeTextureSlicer>,
    ldtk_maps: Res<Assets<LdtkMap>>,
) -> Result {
    let dialog_layout = &texture_atlas_layout_handles.dialog;
    let dialog = &game_asset.interface.dialog;
    let gray_button_slicer = &dialog_texture_slicer.gray_button;

    let chrome = &game_asset.interface.chrome;
    let chrome_layout = &texture_atlas_layout_handles.chrome;
    let map_viewer_slicer = &chrome_texture_slicer.map_viewer;

    let map = ldtk_maps
        .get(
            game_asset
                .maps
                .first()
                .ok_or("No maps available in game assets")?,
        )
        .ok_or("Requested map not found in ldtk_maps")?;

    let map_test = (
        Node {
            border: UiRect::all(Val::Px(10.)),
            width: Val::Px(100.),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ImageNode::new(map.thumbnail.clone()),
    );
    commands.spawn((SelectMapSceneMark, Camera2d));
    commands.spawn((
        SelectMapSceneMark,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        BackgroundColor(Color::Srgba(GRAY)),
        children![(
            Node {
                align_items: AlignItems::Stretch,
                width: Val::Percent(80.),
                height:Val::Percent(80.),
                ..Default::default()
            },
            ImageNode::from_atlas_image(
                chrome.clone(),
                TextureAtlas {
                    layout: chrome_layout.clone(),
                    index: ChromeAtlasKind::MapViewer as usize,
                },
            )
            .with_mode(NodeImageMode::Sliced(map_viewer_slicer.clone())),
            children![
                (
                    Node {
                        align_items: AlignItems::Stretch,
                        justify_content: JustifyContent::SpaceAround,
                        align_self: AlignSelf::FlexEnd,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(65.3),
                        ..Default::default()
                    },
                    children![map_test.clone(), map_test.clone(), map_test.clone()]
                ),
                (Node {
                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::SpaceAround,
                    align_self: AlignSelf::FlexEnd,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(34.7),
                    ..Default::default()
                },)
            ],
        )],
    ));

    Ok(())
}
