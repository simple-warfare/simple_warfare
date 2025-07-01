use crate::{
    assets::{
        GameAsset,
        texture::{
            TextureAtlasLayoutHandles,
            interface::{DialogAtlasKind, DialogTextureSlicer},
        },
    },
    bevy_ext::app::AppExt,
    mod_engine::server::ModServer,
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
        .load("maps/BaiCai's Water Ring Lake/BaiCai's Water Ring Lake.ldtk")
        .into();

    commands.spawn((
        MainSceneMark,
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
    mod_server: Res<ModServer>,
) {
    commands.spawn((
        MainSceneMark,
        Camera2d,
        IsDefaultUiCamera,
        RenderLayers::layer(1),
        FlyCamera2d::default(),
    ));
    let map_camera = commands
        .spawn((
            MainSceneMark,
            Camera2d,
            Camera {
                order: 1,
                ..default()
            },
        ))
        .id();
    let dialog_layout = &texture_atlas_layout_handles.dialog;
    let dialog = &game_asset.interface.dialog;
    let main_menu_slicer = &dialog_texture_slicer.main_menu;
    let gray_button_slicer = &dialog_texture_slicer.gray_button;

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
                    layout: dialog_layout.clone(),
                    index: DialogAtlasKind::GrayButton as usize,
                },
            )
            .with_mode(NodeImageMode::Sliced(gray_button_slicer.clone())),
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
    ));

    commands.spawn((
        MainSceneMark,
        UiTargetCamera(map_camera),
        Node {
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::SpaceAround,
            align_self: AlignSelf::FlexEnd,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(10.)),
            padding: UiRect::all(Val::Px(10.)),
            margin: UiRect {
                left: Val::Px(50.),
                bottom: Val::Percent(5.),
                ..Default::default()
            },
            row_gap: Val::Px(20.),
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
            create_button("SinglePlayer", ButtonLabel::SinglePlayer),
            create_button("MultipPlayer", ButtonLabel::MultipPlayer),
            create_button("Setting", ButtonLabel::Setting),
            create_button("News", ButtonLabel::News),
            create_button("Quit", ButtonLabel::Quit)
        ],
    ));

    //mod_server.spawn_unit(commands.spawn_empty().id(), "example:Tank");
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
                exit_event.write(AppExit::Success);
            }
        }
    }
}
