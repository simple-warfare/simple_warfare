use bevy::prelude::*;

use crate::{app_state::AppState, assets::GameAsset, bevy_ext::app::AppExt};
use bevy_seedling::prelude::*;
use super::{Scene, SceneState};

#[derive(Default)]
pub struct LoadingScene;

#[derive(Component)]
struct LoadingSceneMark;

impl Scene for LoadingScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<LoadingSceneMark, _, _>(SceneState::LoadingScene, setup)
            .add_systems(
                Update,
                (|mut scene_state: ResMut<NextState<SceneState>>| {
                    scene_state.set(SceneState::MainScene);
                })
                .run_if(in_state(AppState::MainLuaExecuted).and(run_once)),
            );
    }
}

fn setup(mut commands: Commands,asset_server:Res<AssetServer>, game_asset: Res<GameAsset>) {
    let mut background_music = SamplePlayer::new(asset_server.load("music/background/war.mp3"));
    background_music.repeat_mode = RepeatMode::RepeatEndlessly;
    commands.spawn(background_music);
    commands.spawn((
        LoadingSceneMark,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_self: JustifySelf::Center,
            justify_content: JustifyContent::Center,
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
                image: game_asset.interface.loading_screen.clone(),
                ..Default::default()
            },
        )],
    ));
}
