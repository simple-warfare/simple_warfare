use bevy::prelude::*;

pub const MOD_SET_PATH: &'static str = "mod_set/";
pub const MOD_SET_NOW_USE_CONF_PATH: &'static str = "mod_set/now_use.conf";
pub const CUSTOM_MOD_PATH: &'static str = "mods/custom/";
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

pub struct StatistcsPlugin;

impl Plugin for StatistcsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Statistics>().init_state::<MouseState>();
    }
}
