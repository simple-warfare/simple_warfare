use bevy::prelude::*;

use crate::{app_state::AppState, bevy_ext::app::AppExt};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct LoadingScene;

#[derive(Component)]
struct LoadingSceneMark;

impl Scene for LoadingScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<LoadingSceneMark, _>(SceneState::LoadingScene, setup)
            .add_systems(
                Update,
                (|mut scene_state: ResMut<NextState<SceneState>>| {
                    scene_state.set(SceneState::MainScene);
                })
                .run_if(in_state(AppState::MainLuaExecuted).and(run_once)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading Scene");
    commands.spawn((
        LoadingSceneMark,
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
            Node{
                height:Val::Percent(150.),
                ..Default::default()
            },
            ImageNode {
            image: asset_server.load("interface/loadscreen.png"),
            ..Default::default()
        },)],
    ));
}
