use bevy::prelude::*;

#[derive(States, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy,Default)]
pub enum AppState {
    #[default]
    AssetsLoading,
    AssetsProcessing,
    LibsLoading,
    LibsLoaded,
    ModInfoLoading,
    ModInfoLoaded,
    MainLuaExecuting,
    MainLuaExecuted,
}
