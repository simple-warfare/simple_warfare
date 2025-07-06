use bevy::prelude::*;
use bevy_fly_camera::FlyCamera2d;

use crate::{assets::GameAsset, bevy_ext::app::AppExt, mod_engine::server::ModServer};

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

fn setup(mut commands: Commands, _game_asset: Res<GameAsset>, mod_server: Res<ModServer>) {
    commands.spawn((GameSceneMark, Camera2d, FlyCamera2d::default()));
    //mod_server.spawn_unit(commands.spawn_empty().id(), "example:Tank");
    mod_server.spawn_unit(commands.spawn_empty().id(), "example:Builder");
}
