use super::{Scene, SceneState};
use crate::{bevy_ext::app::AppExt, panel::main_menu::MainMenuState};
use bevy::prelude::*;

#[derive(Default)]
pub struct MainScene;

#[derive(Component)]
struct MainSceneMark;

impl Scene for MainScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<MainSceneMark, _, _>(SceneState::MainScene, setup);
    }
}

//fn background_map(mut commands: Commands, asset_server: Res<AssetServer>) {}
fn setup(mut main_menu_state: ResMut<NextState<MainMenuState>>) {
    main_menu_state.set(MainMenuState::First);
}
