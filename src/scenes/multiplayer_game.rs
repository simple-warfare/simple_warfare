use crate::{
    assets::{
        GameAsset,
        texture::{
            TextureAtlasLayoutHandles,
            dialog::{DialogAtlasKind, DialogTextureSlicer},
        },
    },
    bevy_ext::app::AppExt,
};
use bevy::{color::palettes::css::WHITE, prelude::*};
use serde::{Deserialize, Serialize};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct MultiplayerGame;

#[derive(Component)]
struct MultiplayerGameMark;

impl Scene for MultiplayerGame {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<MultiplayerGameMark, _, _>(
            SceneState::MultiplayerGame,
            (setup, add_observer_for_button).chain(),
        );
    }
}

#[derive(Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
enum ButtonLabel {
    ChangeMap,
    FilterGames,
    DirectIp,
    Create,
}

fn setup(
    mut commands: Commands,
    game_asset: Res<GameAsset>,
    texture_atlas_layout_handles: Res<TextureAtlasLayoutHandles>,
    dialog_texture_slicer: Res<DialogTextureSlicer>,
) {
    let main_menu_slicer = &dialog_texture_slicer.main_menu;
    let dialog_layout = &texture_atlas_layout_handles.dialog;
    let dialog = &game_asset.interface.dialog;
    let dark_gray_rect_slicer = &dialog_texture_slicer.dark_gray_rect;

    let gray_rect_slicer = &dialog_texture_slicer.gray_rect;

    let create_text = |text: &str, font_size: f32| {
        (
            Text::new(text),
            TextFont::from_font_size(font_size),
            TextLayout::new_with_justify(JustifyText::Center),
            TextColor(Color::Srgba(WHITE)),
        )
    };

    let create_button = |text: &str, font_size: f32, lable: ButtonLabel| {
        (
            Node {
                display: Display::Grid,
                border: UiRect::all(Val::Px(8.)),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..Default::default()
            },
            lable,
            Button,
            ImageNode::from_atlas_image(
                dialog.clone(),
                TextureAtlas {
                    layout: dialog_layout.clone(),
                    index: DialogAtlasKind::GrayRect as usize,
                },
            )
            .with_mode(NodeImageMode::Sliced(gray_rect_slicer.clone())),
            children![(
                Text::new(text),
                TextLayout::new_with_justify(JustifyText::Center),
                TextFont::from_font_size(font_size),
                TextColor(Color::Srgba(WHITE),)
            )],
        )
    };

    commands.spawn((
        MultiplayerGameMark,
        Node {
            display: Display::Grid,
            width: Val::Percent(70.),
            height: Val::Percent(65.),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            padding: UiRect::all(Val::Px(10.)),
            border: UiRect::all(Val::Px(4.)),
            row_gap: Val::Px(5.),
            column_gap: Val::Px(5.),
            grid_template_rows: vec![
                GridTrack::flex(1.),
                GridTrack::flex(10.),
                GridTrack::flex(10.),
                GridTrack::flex(1.),
            ],
            grid_template_columns: vec![GridTrack::flex(3.), GridTrack::flex(1.)],
            ..Default::default()
        },
        ImageNode::from_atlas_image(
            dialog.clone(),
            TextureAtlas {
                layout: dialog_layout.clone(),
                index: DialogAtlasKind::MainMenu as usize,
            },
        )
        .with_mode(NodeImageMode::Sliced(main_menu_slicer.clone())),
        children![
            (
                Node {
                    display: Display::Grid,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    grid_column: GridPlacement::span(2),
                    ..Default::default()
                },
                create_text("Multiplayer", 20.)
            ),
            (
                Node {
                    display: Display::Grid,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    grid_auto_columns: vec![
                        GridTrack::min_content(),
                        GridTrack::max_content(),
                        GridTrack::min_content()
                    ],
                    ..Default::default()
                },
                children![
                    (
                        Node {
                            display: Display::Grid,
                            grid_template_columns: RepeatedGridTrack::flex(4, 1.),
                            column_gap: Val::Px(2.),
                            ..Default::default()
                        },
                        children![
                            create_text("Server", 10.),
                            create_text("Players", 10.),
                            create_text("Location", 10.),
                            create_text("Status", 10.),
                        ]
                    ),
                    (
                        Node {
                            display: Display::Grid,
                            width: Val::Percent(100.),
                            overflow: Overflow::scroll_y(),
                            ..Default::default()
                        },
                        ImageNode::from_atlas_image(
                            dialog.clone(),
                            TextureAtlas {
                                layout: dialog_layout.clone(),
                                index: DialogAtlasKind::DarkGrayRect2 as usize,
                            },
                        )
                        .with_mode(NodeImageMode::Sliced(dark_gray_rect_slicer.clone())),
                    ),
                    (
                        Node {
                            display: Display::Grid,
                            grid_template_columns: RepeatedGridTrack::flex(4, 1.),
                            column_gap: Val::Px(2.),
                            ..Default::default()
                        },
                        children![
                            create_text("Server", 10.),
                            create_text("Players", 10.),
                            create_text("Location", 10.),
                            create_text("Status", 10.),
                        ]
                    ),
                ]
            ),
            (
                Node {
                    display: Display::Grid,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    grid_template_rows: vec![
                        GridTrack::flex(8.),
                        GridTrack::flex(1.),
                        GridTrack::flex(2.),
                    ],
                    column_gap: Val::Px(5.),
                    ..Default::default()
                },
                children![
                    (
                        Node {
                            display: Display::Grid,
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            border: UiRect::all(Val::Px(10.)),
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::Center,
                            ..Default::default()
                        },
                        ImageNode::from_atlas_image(
                            dialog.clone(),
                            TextureAtlas {
                                layout: dialog_layout.clone(),
                                index: DialogAtlasKind::DarkGrayRect2 as usize,
                            },
                        )
                        .with_mode(NodeImageMode::Sliced(dark_gray_rect_slicer.clone())),
                        children![(Node {
                            display: Display::Grid,
                            max_width: Val::Percent(100.),
                            max_height: Val::Percent(100.),
                            aspect_ratio: Some(1.),
                            justify_self: JustifySelf::Center,
                            ..Default::default()
                        },)]
                    ),
                    (
                        Node {
                            display: Display::Grid,
                            max_width: Val::Percent(100.),
                            max_height: Val::Percent(100.),
                            justify_self: JustifySelf::Center,
                            ..Default::default()
                        },
                        create_text("No Server Selected", 10.)
                    ),
                    create_button("Change Map", 15., ButtonLabel::ChangeMap)
                ]
            ),
            (
                Node {
                    display: Display::Grid,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    margin: UiRect::top(Val::Px(10.)),
                    grid_column: GridPlacement::span(2),
                    grid_template_rows: vec![GridTrack::flex(8.), GridTrack::flex(2.),],
                    column_gap: Val::Px(5.),
                    ..Default::default()
                },
                children![(
                    Node {
                        display: Display::Grid,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        overflow: Overflow::scroll_y(),
                        ..Default::default()
                    },
                    ImageNode::from_atlas_image(
                        dialog.clone(),
                        TextureAtlas {
                            layout: dialog_layout.clone(),
                            index: DialogAtlasKind::DarkGrayRect2 as usize,
                        },
                    )
                    .with_mode(NodeImageMode::Sliced(dark_gray_rect_slicer.clone())),
                ),]
            )
        ],
    ));
}

fn observer_click(
    click: Trigger<Pointer<Click>>,
    buttons: Query<&ButtonLabel, With<Button>>,
    mut scene_state: ResMut<NextState<SceneState>>,
) {
    if let Ok(lable) = buttons.get(click.target()) {
        match lable {
            ButtonLabel::ChangeMap => scene_state.set(SceneState::SelectMapScene),
            ButtonLabel::FilterGames => todo!(),
            ButtonLabel::DirectIp => todo!(),
            ButtonLabel::Create => todo!(),
        }
    }
}

fn add_observer_for_button(mut commands: Commands, buttons: Query<Entity, With<ButtonLabel>>) {
    buttons.iter().for_each(|btn_entity| {
        commands.entity(btn_entity).observe(observer_click);
    });
}

fn refresh_lobby() {}
