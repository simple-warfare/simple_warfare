mod loading;
mod mian;
use bevy::prelude::*;

use crate::{bevy_ext::app::AppExt, scenes::{loading::LoadingScene, mian::MainScene}};

pub trait Scene: Default {
    fn build(&self, app: &mut App);
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum SceneState {
    #[default]
    LoadingScene,
    MainScene,
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SceneState>()
        .init_scene::<LoadingScene>()
        .init_scene::<MainScene>();
    }
}
