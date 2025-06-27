use bevy::prelude::*;

use crate::{app_state::AppState, assets::assets::GameAssets, bevy_ext::app::AppExt};

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

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        MainSceneMark,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_self: JustifySelf::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        BackgroundColor(Color::BLACK),
        children![(
            Node {
                height: Val::Percent(150.),
                ..Default::default()
            },
            ImageNode {
                image: game_assets.interfaces.loadscreen.clone(),
                ..Default::default()
            },
        )],
    ));
}
