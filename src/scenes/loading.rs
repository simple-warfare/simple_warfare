use bevy::{prelude::*, render::view::RenderLayers};
use bevy_fly_camera::FlyCamera2d;

use crate::{app_state::AppState, assets::GameAsset, bevy_ext::app::AppExt};

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

fn setup(mut commands: Commands, game_asset: Res<GameAsset>) {
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
