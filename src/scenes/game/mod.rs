pub mod input;

use bevy::prelude::*;
use bevy_fly_camera::FlyCamera2d;

use crate::{
    assets::GameAsset, bevy_ext::app::AppExt, mod_engine::server::ModServer, scenes::game::input::*,
};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct GameScene;

#[derive(Component)]
struct GameSceneMark;

impl Scene for GameScene {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .init_resource::<MousePosition>()
            .add_scene_system::<GameSceneMark, _, _>(SceneState::GameScene, setup)
            .add_systems(
                FixedUpdate,
                (
                    handle_cursor_move,
                    handle_mouse_input,
                    calculate_world_position_of_selection,
                    updata_selected_unit,
                    draw_selection_box,
                    draw_selected_unit,
                )
                    .chain()
                    .run_if(in_state(SceneState::GameScene)),
            )
            .add_systems(
                FixedUpdate,
                (test_move, test_handle_active_way_point_move)
                    .run_if(in_state(SceneState::GameScene)),
            );
    }
}

fn setup(mut commands: Commands, game_asset: Res<GameAsset>, mod_server: Res<ModServer>) {
    commands.spawn((GameSceneMark, Camera2d, FlyCamera2d::default()));
    mod_server.spawn_unit(commands.spawn_empty().id(), "example:Tank");
}
