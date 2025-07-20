pub mod game;
pub mod loading;
pub mod mian;
pub mod select_map;
pub mod skirmish_game;

use bevy::prelude::*;

use crate::{
    bevy_ext::app::AppExt,
    scenes::{
        game::GameScene, loading::LoadingScene, mian::MainScene, select_map::SelectMapScene,
        skirmish_game::SkirmishGame,
    },
};

pub trait Scene: Default {
    fn build(&self, app: &mut App);
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum SceneState {
    #[default]
    LoadingScene,
    MainScene,
    SelectMapScene,
    GameScene,
    SkirmishGame,
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SceneState>()
            .init_scene::<LoadingScene>()
            .init_scene::<MainScene>()
            .init_scene::<SelectMapScene>()
            .init_scene::<GameScene>()
            .init_scene::<SkirmishGame>();
    }
}
