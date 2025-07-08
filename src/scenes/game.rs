use crate::{assets::GameAsset, bevy_ext::app::AppExt, mod_engine::server::ModServer};
use bevy::prelude::*;
use bevy_fly_camera::FlyCamera2d;

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
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 1.;
    commands.spawn((
        GameSceneMark,
        Camera2d,
        FlyCamera2d::default(),
        Projection::Orthographic(projection),
    ));

    mod_server.spawn_unit("example:Tank");
    mod_server.spawn_unit("example:Tank");
    //mod_server.spawn_unit("example:Tank");
    //mod_server.spawn_unit(commands.spawn_empty().id(), "example:Builder");
}
