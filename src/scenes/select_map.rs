use bevy::{color::palettes::css::*, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    assets::{
        GameAsset,
        map::ldtk::LdtkMap,
        texture::{
            TextureAtlasLayoutHandles,
            chrome::{ChromeAtlasKind, ChromeTextureSlicer},
            dialog::{DialogAtlasKind, DialogTextureSlicer},
        },
    },
    bevy_ext::app::AppExt,
};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct SelectMapScene;

#[derive(Component)]
struct SelectMapSceneMark;

#[derive(Component, Clone)]
struct MapRect;

impl Scene for SelectMapScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<SelectMapSceneMark, _, _>(
            SceneState::SelectMapScene,
            (setup, show_map).chain(),
        );
    }
}

fn setup(
    mut commands: Commands,
    game_asset: Res<GameAsset>,
    texture_atlas_layout_handles: Res<TextureAtlasLayoutHandles>,
    dialog_texture_slicer: Res<DialogTextureSlicer>,
    chrome_texture_slicer: Res<ChromeTextureSlicer>,
) -> Result {
    let dialog_layout = &texture_atlas_layout_handles.dialog;
    let dialog = &game_asset.interface.dialog;
    let light_blue_rect_slicer = &dialog_texture_slicer.light_blue_rect;

    let chrome = &game_asset.interface.chrome;
    let chrome_layout = &texture_atlas_layout_handles.chrome;
    let map_viewer_slicer = &chrome_texture_slicer.map_viewer;

    let map_rect = (
        MapRect,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Grid,
            border: UiRect::all(Val::Px(10.)),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        Button,
        ImageNode::from_atlas_image(
            dialog.clone(),
            TextureAtlas {
                layout: dialog_layout.clone(),
                index: DialogAtlasKind::LightBlueRect as usize,
            },
        )
        .with_mode(NodeImageMode::Sliced(light_blue_rect_slicer.clone())),
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
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(2.0), GridTrack::flex(1.0)],
                grid_template_rows: vec![
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                ],
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
                        aspect_ratio: Some(1.0),
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        display: Display::Grid,
                        padding: UiRect::all(Val::Px(24.0)),
                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                        grid_row: GridPlacement::span(3),
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..Default::default()
                    },
                    children![
                        map_rect.clone(),
                        map_rect.clone(),
                        map_rect.clone(),
                        map_rect.clone(),
                        map_rect.clone(),
                        map_rect.clone(),
                        map_rect.clone(),
                        map_rect.clone(),
                        map_rect.clone(),
                    ]
                ),
                (Node {
                    align_items: AlignItems::Start,
                    justify_items: JustifyItems::Center,
                    padding: UiRect::all(Val::Px(10.)),
                    grid_template_rows: vec![
                        GridTrack::auto(),
                        GridTrack::auto(),
                        GridTrack::fr(1.0)
                    ],
                    row_gap: Val::Px(10.),
                    ..Default::default()
                },)
            ],
        )],
    ));

    Ok(())
}

fn show_map(
    mut commands: Commands,
    ldtk_maps: Res<Assets<LdtkMap>>,
    game_asset: Res<GameAsset>,
    map_rects: Query<Entity, With<MapRect>>,
) -> Result {
    let text_style = TextFont::default();
    let map = ldtk_maps
        .get(
            game_asset
                .maps
                .first()
                .ok_or("No maps available in game assets")?,
        )
        .ok_or("Requested map not found in ldtk_maps")?;

    for entity in map_rects {
        commands.entity(entity).with_child((
            Node {
                max_width: Val::Percent(100.),
                max_height: Val::Percent(100.),
                aspect_ratio: Some(1.),
                border: UiRect::all(Val::Px(10.)),
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ImageNode::new(map.thumbnail.clone()),
        ));
    }

    Ok(())
}
