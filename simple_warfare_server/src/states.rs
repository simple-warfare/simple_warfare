pub mod gaming;
pub mod loading;

use bevy::prelude::*;
use self::loading::LoadingStatePlugin;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum GameBackendState {
    #[default]
    Loading,
    Lobby,
}

pub struct ServerStatePlugin;

impl Plugin for ServerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingStatePlugin);
    }
}
