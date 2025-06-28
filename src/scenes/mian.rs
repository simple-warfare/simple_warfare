use bevy::{color::palettes::css::*, prelude::*};

use crate::{
    app_state::AppState,
    assets::{
        GameAsset,
        texture::{TextureAtlasLayoutHandles, interface::DialogTextureSlicer},
    },
    bevy_ext::app::AppExt,
};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct MainScene;

#[derive(Component)]
struct MainSceneMark;

impl Scene for MainScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<MainSceneMark, _>(SceneState::MainScene, setup);
    }
}

fn setup(
    mut commands: Commands,
    game_asset: Res<GameAsset>,
    texture_atlas_layout_handles: Res<TextureAtlasLayoutHandles>,
    dialog_texture_slicer: Res<DialogTextureSlicer>,
) {
    let layout = &texture_atlas_layout_handles.dialog;
    let dialog = &game_asset.interface.dialog;
    let main_menu_slicer = &dialog_texture_slicer.main_menu_slicer;
    let main_button_slicer = &dialog_texture_slicer.main_button_slicer;

    let text_style = TextFont::default();

    let create_button = |text: &str| {
        (
            Node {
                border: UiRect::all(Val::Px(10.)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            Button,
            ImageNode::from_atlas_image(
                dialog.clone(),
                TextureAtlas {
                    layout: layout.clone(),
                    index: 1,
                },
            )
            .with_mode(NodeImageMode::Sliced(main_button_slicer.clone())),
            children![(
                Text::new(text),
                text_style.clone(),
                TextColor(Color::Srgba(WHITE),)
            )],
        )
    };
    commands.spawn((
        MainSceneMark,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_self: JustifySelf::Center,
            align_items: AlignItems::End,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        BackgroundColor(Color::Srgba(GRAY)),
        children![(
            Node {
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::SpaceAround,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(10.)),
                padding: UiRect::all(Val::Px(10.)),
                margin: UiRect {
                    left: Val::Percent(3.),
                    bottom: Val::Percent(10.),
                    ..Default::default()
                },
                row_gap:Val::Px(20.),
                ..Default::default()
            },
            ImageNode::from_atlas_image(
                dialog.clone(),
                TextureAtlas {
                    layout: layout.clone(),
                    index: 0
                },
            )
            .with_mode(NodeImageMode::Sliced(main_menu_slicer.clone())),
            children![
                create_button("Single Player"),
                create_button("Multip Player"),
                create_button("Setting"),
                create_button("News"),
                create_button("Quit")
            ]
        ),],
    ));
}
