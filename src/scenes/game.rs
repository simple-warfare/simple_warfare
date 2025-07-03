use bevy::{prelude::*, render::view::RenderLayers};
use bevy_fly_camera::FlyCamera2d;

use crate::{app_state::AppState, assets::GameAsset, bevy_ext::app::AppExt};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct GameScene;

#[derive(Component)]
struct GameSceneMark;

impl Scene for GameScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<GameSceneMark, _, _>(SceneState::GameScene, setup);
    }
}

fn setup(mut commands: Commands, game_asset: Res<GameAsset>) {
    commands.spawn((
        GameSceneMark,
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
    ));
}
