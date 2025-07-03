use bevy::prelude::*;

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

pub struct StatistcsPlugin;

impl Plugin for StatistcsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Statistics>();
    }
}
