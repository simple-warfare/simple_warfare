use crate::{
    assets::{
        GameAsset,
        texture::{TextureAtlasLayoutHandles, interface::DialogTextureSlicer},
    },
    bevy_ext::app::AppExt,
};
use bevy::{color::palettes::css::*, prelude::*, render::view::RenderLayers};
use bevy_ecs_ldtk::prelude::*;
use bevy_fly_camera::FlyCamera2d;
use serde::{Deserialize, Serialize};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct MainScene;

#[derive(Component)]
struct MainSceneMark;

#[derive(Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub enum ButtonLabel {
    SinglePlayer,
    MultipPlayer,
    Setting,
    News,
    Quit,
}

impl Scene for MainScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<MainSceneMark, _, _>(SceneState::MainScene, (background_map, setup))
            .add_observer(button_click);
    }
}
fn background_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_handle = asset_server
        .load("map/BaiCai's Water Ring Lake/BaiCai's Water Ring Lake.ldtk")
        .into();

    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle,
            ..Default::default()
        },
    ));
}
fn setup(
    mut commands: Commands,
    game_asset: Res<GameAsset>,
    texture_atlas_layout_handles: Res<TextureAtlasLayoutHandles>,
    dialog_texture_slicer: Res<DialogTextureSlicer>,
) {
    commands.spawn((Camera2d, IsDefaultUiCamera, FlyCamera2d::default()));

    let layout = &texture_atlas_layout_handles.dialog;
    let dialog = &game_asset.interface.dialog;
    let main_menu_slicer = &dialog_texture_slicer.main_menu_slicer;
    let main_button_slicer = &dialog_texture_slicer.main_button_slicer;

    let text_style = TextFont::default();

    let create_button = |text: &str, lable: ButtonLabel| {
        (
            Node {
                border: UiRect::all(Val::Px(10.)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            lable,
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
                row_gap: Val::Px(20.),
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
                create_button("SinglePlayer", ButtonLabel::SinglePlayer),
                create_button("MultipPlayer", ButtonLabel::MultipPlayer),
                create_button("Setting", ButtonLabel::Setting),
                create_button("News", ButtonLabel::News),
                create_button("Quit", ButtonLabel::Quit)
            ]
        ),],
    ));
}

/// An observer to rotate an entity when it is dragged
fn button_click(
    click: Trigger<Pointer<Click>>,
    buttons: Query<&ButtonLabel, With<Button>>,
    mut exit_event: EventWriter<AppExit>,
    mut scene_state: ResMut<NextState<SceneState>>,
) {
    if let Ok(lable) = buttons.get(click.target()) {
        match lable {
            ButtonLabel::SinglePlayer => {
                scene_state.set(SceneState::SelectMapScene);
            }
            ButtonLabel::MultipPlayer => todo!(),
            ButtonLabel::Setting => todo!(),
            ButtonLabel::News => todo!(),
            ButtonLabel::Quit => {
                info!("AppExit");
                exit_event.write(AppExit::Success);
            }
        }
    }
}
