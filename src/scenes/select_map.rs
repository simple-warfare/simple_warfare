use std::sync::Arc;

use bevy::{color::palettes::css::*, prelude::*};
use serde::{Deserialize, Serialize};
use url::form_urlencoded::Target;

use crate::{
    assets::{
        GameAsset,
        map::{
            SimpleWarfareMap,
            ldtk::LdtkMap,
            tiled::{TiledMap, TiledMapInfo},
        },
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

#[derive(Component)]
struct MapGridContainer;

impl Scene for SelectMapScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<SelectMapSceneMark, _, _>(
            SceneState::SelectMapScene,
            (setup, add_observer_for_button, show_map).chain(),
        )
        .add_observer(observer_click);
    }
}

#[derive(Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
enum ButtonLabel {
    Ok,
}

#[derive(Debug, Component)]
pub struct MapTitleViewer;

fn setup(
    mut commands: Commands,
    game_asset: Res<GameAsset>,
    texture_atlas_layout_handles: Res<TextureAtlasLayoutHandles>,
    chrome_texture_slicer: Res<ChromeTextureSlicer>,
) -> Result {
    let chrome = &game_asset.interface.chrome;
    let chrome_layout = &texture_atlas_layout_handles.chrome;
    let map_viewer_slicer = &chrome_texture_slicer.map_viewer;
    let gray_brick_rect = &chrome_texture_slicer.gray_brick_rect;

    commands.spawn((SelectMapSceneMark, Camera2d));
    commands.spawn((
        SelectMapSceneMark,
        Node {
            display: Display::Grid,
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
                display: Display::Grid,
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                grid_template_columns: vec![GridTrack::flex(1.9), GridTrack::flex(1.0)],
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
                    MapGridContainer,
                    Node {
                        display: Display::Grid,
                        aspect_ratio: Some(1.0),
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        padding: UiRect::all(Val::Px(18.0)),
                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                        grid_row: GridPlacement::span(3),
                        row_gap: Val::Px(6.0),
                        column_gap: Val::Px(6.0),
                        ..Default::default()
                    },
                ),
                (
                    Node {
                        display: Display::Grid,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        padding: UiRect::all(Val::Px(10.)),
                        grid_template_rows: vec![
                            GridTrack::fr(2.0),
                            GridTrack::fr(5.0),
                            GridTrack::fr(1.0)
                        ],
                        row_gap: Val::Px(10.),
                        ..Default::default()
                    },
                    children![
                        (
                            Node {
                                display: Display::Grid,
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                ..Default::default()
                            },
                            children![
                                MapTitleViewer,
                                Text::default(),
                                TextFont::from_font_size(10.),
                                TextLayout {
                                    justify: JustifyText::Center,
                                    linebreak: LineBreak::AnyCharacter
                                },
                                TextColor(Color::Srgba(BLACK),)
                            ]
                        ),
                        (Node {
                            display: Display::Grid,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..Default::default()
                        }),
                        (
                            Node {
                                display: Display::Grid,
                                align_items: AlignItems::Center,
                                justify_items: JustifyItems::Center,
                                ..Default::default()
                            },
                            Button,
                            ButtonLabel::Ok,
                            ImageNode::from_atlas_image(
                                chrome.clone(),
                                TextureAtlas {
                                    layout: chrome_layout.clone(),
                                    index: ChromeAtlasKind::GrayBrickRect as usize
                                }
                            )
                            .with_mode(NodeImageMode::Sliced(gray_brick_rect.clone())),
                            children![(Text::new("Ok"), TextColor(Color::Srgba(WHITE),))]
                        )
                    ]
                )
            ],
        )],
    ));

    Ok(())
}

#[derive(Debug, Component, Clone)]
pub struct MapPointer(Arc<SimpleWarfareMap>);

fn show_map(
    mut commands: Commands,
    ldtk_maps: Res<Assets<LdtkMap>>,
    tiled_maps: Res<Assets<TiledMap>>,
    tiled_map_infos: Res<Assets<TiledMapInfo>>,
    game_asset: Res<GameAsset>,
    texture_atlas_layout_handles: Res<TextureAtlasLayoutHandles>,
    dialog_texture_slicer: Res<DialogTextureSlicer>,
    map_grid_container: Single<Entity, With<MapGridContainer>>,
) -> Result {
    let dialog_layout = &texture_atlas_layout_handles.dialog;
    let dialog = &game_asset.interface.dialog;
    let light_blue_rect_slicer = &dialog_texture_slicer.light_blue_rect;
    let text_style = TextFont::from_font_size(10.);

    let create_map = |map: Arc<SimpleWarfareMap>| match map.as_ref() {
        SimpleWarfareMap::Ldtk(map_handle) => todo!(),
        SimpleWarfareMap::Tiled(map_handle) => {
            let tiled_map = tiled_maps.get(map_handle.id()).unwrap();
            let tiled_map_info = tiled_map_infos.get(tiled_map.info.id()).unwrap();

            let thumbnail = tiled_map
                .thumbnail
                .clone()
                .unwrap_or(game_asset.interface.missing_map_thumbnail.clone());
            (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: UiRect::all(Val::Px(6.)),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                Button,
                MapPointer(map.clone()),
                ImageNode::from_atlas_image(
                    dialog.clone(),
                    TextureAtlas {
                        layout: dialog_layout.clone(),
                        index: DialogAtlasKind::LightBlueRect as usize,
                    },
                )
                .with_mode(NodeImageMode::Sliced(light_blue_rect_slicer.clone())),
                children![
                    (
                        Node {
                            width: Val::Percent(90.),
                            max_height: Val::Percent(80.),
                            aspect_ratio: Some(1.),
                            border: UiRect::all(Val::Px(10.)),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        ImageNode::new(thumbnail.clone()),
                    ),
                    (
                        Node {
                            width: Val::Percent(90.),
                            max_height: Val::Percent(20.),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        Text::new(&tiled_map_info.title),
                        text_style.clone(),
                        TextLayout {
                            justify: JustifyText::Center,
                            linebreak: LineBreak::AnyCharacter
                        },
                        TextColor(Color::Srgba(WHITE),)
                    )
                ],
            )
        }
    };

    commands
        .entity(map_grid_container.entity())
        .with_children(|container| {
            game_asset.maps.iter().for_each(|map| {
                container
                    .spawn(create_map(map.clone()))
                    .observe(observer_click);
            });
        });
    Ok(())
}

fn observer_click(
    click: Trigger<Pointer<Click>>,
    tiled_maps: Res<Assets<TiledMap>>,
    tiled_map_infos: Res<Assets<TiledMapInfo>>,
    map_pointers: Query<&MapPointer>,
    mut map_title_viewer: Single<&mut Text, With<MapTitleViewer>>,
    buttons: Query<&ButtonLabel, With<Button>>,
    mut scene_state: ResMut<NextState<SceneState>>,
) {
    info!("observer_click");
    if let Ok(map_pointer) = map_pointers.get(click.target()) {
        info!("map_pointer");
        match map_pointer.0.as_ref() {
            SimpleWarfareMap::Ldtk(handle) => todo!(),
            SimpleWarfareMap::Tiled(map_handle) => {
                let tiled_map = tiled_maps.get(map_handle.id()).unwrap();
                let tiled_map_info = tiled_map_infos.get(tiled_map.info.id()).unwrap();
                *map_title_viewer.as_deref_mut() = tiled_map_info.title.clone();
            }
        }
    } else if let Ok(lable) = buttons.get(click.target()) {
        match lable {
            ButtonLabel::Ok => scene_state.set(SceneState::GameScene),
        }
    }
}

fn add_observer_for_button(mut commands: Commands, buttons: Query<Entity, With<ButtonLabel>>) {
    buttons.iter().for_each(|btn_entity| {
        info!("add_observer_for_button");
        commands.entity(btn_entity).observe(observer_click);
    });
}
