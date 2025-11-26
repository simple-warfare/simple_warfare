use std::sync::{Arc, atomic::AtomicU8};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::consts::{GAME_VERSION, GAME_VERSION_TYPE};

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

pub const SOME_ASYNC_WORK_NUM: u8 = 2;

#[derive(Debug, Default, Resource)]
pub struct SomeAsyncWorkCalculator(pub Arc<AtomicU8>);

#[derive(States, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Reflect)]
pub enum ServerState {
    #[default]
    StartServerWaiting,
    AssetsLoading,
    JsContextInitiating,
    ModSetLoading,
    CustomModLoading,
    MainLuaExecuting,
    JsFileLoading,
    SomeAsyncWork,
    Starting,
}
#[derive(States, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Reflect)]
pub enum MapBackendState {
    #[default]
    None,
    CreatingMap,
    BuildingNavMesh,
    BuildingNorthStar,
    BuildingFlowField,
}

#[derive(Debug, Component)]
pub struct Selectable;

#[derive(Debug, Component)]
pub struct Selected;

#[derive(Clone, Default, Resource)]
pub struct MousePosition {
    pub viewport: Option<Vec2>,
    pub world_2d: Option<Vec2>,
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
            .init_resource::<SomeAsyncWorkCalculator>()
            .init_state::<MouseState>()
            .init_state::<ServerState>()
            .init_state::<MapBackendState>();
    }
}
