use std::sync::Arc;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    assets::map::SimpleWarfareMap,
    consts::{GAME_VERSION, GAME_VERSION_TYPE},
};

#[derive(Default, Debug, Resource)]
pub struct Statistics {
    pub player_name: String,
    pub game_type: GameType,
}
#[derive(Default, Debug, PartialEq, Eq)]
pub enum GameType {
    #[default]
    SandBox,
}

#[derive(States, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Reflect)]
pub enum NetState {
    #[default]
    None,
    Client,
    Server,
    HostServer,
}

#[derive(Debug, Resource)]
pub struct SelectedMap(pub Arc<SimpleWarfareMap>);

#[derive(Resource, Default, Debug)]
pub struct SelectionState {
    pub start: Vec2,
    pub end: Vec2,
    pub real_start: Vec2,
    pub real_end: Vec2,
    pub is_selecting: bool,
}

#[derive(Debug, Component)]
pub struct Selectable;

#[derive(Debug, Component)]
pub struct Selected;

impl SelectionState {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
#[derive(Clone, Default, Resource)]
pub struct MousePosition {
    pub windows: Option<Vec2>,
    pub world: Option<Vec2>,
}

#[derive(States, Default, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub enum MouseState {
    #[default]
    Nothing,
    Selected,
}

#[derive(Debug, Resource, Serialize, Deserialize, PartialEq, Clone)]
pub struct GameInfo {
    pub game_version: String,
    pub game_version_type: GameVersionType,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GameVersionType {
    Beta,
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            game_version: GAME_VERSION.into(),
            game_version_type: GAME_VERSION_TYPE,
        }
    }
}

pub struct StatistcsPlugin;

impl Plugin for StatistcsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Statistics>()
            .init_resource::<GameInfo>()
            .init_state::<MouseState>()
            .init_state::<NetState>();
    }
}
