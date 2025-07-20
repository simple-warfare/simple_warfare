use crate::{
    assets::{
        GameAsset,
        texture::{
            TextureAtlasLayoutHandles,
            dialog::{DialogAtlasKind, DialogTextureSlicer},
        },
    },
    bevy_ext::app::AppExt,
    scenes::SceneState,
    statistics::SelectedMap,
};
use bevy::{color::palettes::css::*, prelude::*};
use serde::{Deserialize, Serialize};

use super::Panel;
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum MainMenuState {
    #[default]
    Disable,
    First,
    Second,
}

#[derive(Default)]
pub struct MainMenu;

#[derive(Component)]
struct MainMenuMark;

#[derive(Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
enum ButtonLabel {
    SinglePlayer,
    Skirmish,
    MultipPlayer,
    MapEditor,
    News,
    Mods,
    Setting,
    Back,
    Quit,
}

impl Panel for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_panel_system::<MainMenuMark, _, _>(
            MainMenuState::First,
            (first, add_observer_for_button).chain(),
        )
        .add_panel_system::<MainMenuMark, _, _>(
            MainMenuState::Second,
            (second, add_observer_for_button).chain(),
        );
    }
}

fn first(
    mut commands: Commands,
    game_asset: Res<GameAsset>,
    texture_atlas_layout_handles: Res<TextureAtlasLayoutHandles>,
    dialog_texture_slicer: Res<DialogTextureSlicer>,
) {
    let dialog_layout = &texture_atlas_layout_handles.dialog;
    let dialog = &game_asset.interface.dialog;
    let main_menu_slicer = &dialog_texture_slicer.main_menu;
    let gray_rect_slicer = &dialog_texture_slicer.gray_rect;

    let text_style = TextFont::default();

    let create_button = |text: &str, lable: ButtonLabel| {
        (
            Node {
                display: Display::Grid,
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
                    index: DialogAtlasKind::GrayRect as usize,
                },
            )
            .with_mode(NodeImageMode::Sliced(gray_rect_slicer.clone())),
            children![(
                Text::new(text),
                text_style.clone(),
                TextColor(Color::Srgba(WHITE),)
            )],
        )
    };

    commands.spawn((
        MainMenuMark,
        Node {
            display: Display::Grid,
            grid_template_rows: RepeatedGridTrack::flex(7, 1.0),
            align_items: AlignItems::Stretch,
            align_self: AlignSelf::FlexEnd,
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
            create_button("MapEditor", ButtonLabel::MapEditor),
            create_button("Mods", ButtonLabel::Mods),
            create_button("News", ButtonLabel::News),
            create_button("Setting", ButtonLabel::Setting),
            create_button("Quit", ButtonLabel::Quit)
        ],
    ));
}
fn second(
    mut commands: Commands,
    game_asset: Res<GameAsset>,
    texture_atlas_layout_handles: Res<TextureAtlasLayoutHandles>,
    dialog_texture_slicer: Res<DialogTextureSlicer>,
) {
    let dialog_layout = &texture_atlas_layout_handles.dialog;
    let dialog = &game_asset.interface.dialog;
    let main_menu_slicer = &dialog_texture_slicer.main_menu;
    let gray_rect_slicer = &dialog_texture_slicer.gray_rect;

    let text_style = TextFont::default();

    let create_button = |text: &str, lable: ButtonLabel| {
        (
            Node {
                display: Display::Grid,
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
                    index: DialogAtlasKind::GrayRect as usize,
                },
            )
            .with_mode(NodeImageMode::Sliced(gray_rect_slicer.clone())),
            children![(
                Text::new(text),
                text_style.clone(),
                TextColor(Color::Srgba(WHITE),)
            )],
        )
    };

    commands.spawn((
        MainMenuMark,
        Node {
            display: Display::Grid,
            grid_template_rows: RepeatedGridTrack::flex(7, 1.0),
            align_items: AlignItems::Stretch,
            align_self: AlignSelf::FlexEnd,
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
            create_button("Skirmish", ButtonLabel::Skirmish),
            create_button("Back", ButtonLabel::Back)
        ],
    ));
}

fn observer_click(
    click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    buttons: Query<&ButtonLabel, With<Button>>,
    mut exit_event: EventWriter<AppExit>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    game_asset: Res<GameAsset>,
    mut scene_state: ResMut<NextState<SceneState>>,
) {
    if let Ok(lable) = buttons.get(click.target()) {
        match lable {
            ButtonLabel::SinglePlayer => main_menu_state.set(MainMenuState::Second),
            ButtonLabel::MultipPlayer => todo!(),
            ButtonLabel::Setting => todo!(),
            ButtonLabel::News => todo!(),
            ButtonLabel::Quit => {
                exit_event.write(AppExit::Success);
            }
            ButtonLabel::Skirmish => {
                let default_map = game_asset.maps.get(0).unwrap().clone();
                commands.insert_resource(SelectedMap(default_map.clone()));
                main_menu_state.set(MainMenuState::Disable);
                scene_state.set(SceneState::SkirmishGame);
            }
            ButtonLabel::MapEditor => todo!(),
            ButtonLabel::Mods => todo!(),
            ButtonLabel::Back => main_menu_state.set(MainMenuState::First),
        }
    }
}

fn add_observer_for_button(mut commands: Commands, buttons: Query<Entity, With<ButtonLabel>>) {
    buttons.iter().for_each(|btn_entity| {
        commands.entity(btn_entity).observe(observer_click);
    });
}
