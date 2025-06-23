use bevy::prelude::*;

#[derive(States, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy,Default)]
pub enum AppState {
    #[default]
    LibsLoding,
    LibsLoded,
    ModInfoLoading,
    ModInfoLoaded,
    MainLuaExecuting,
    MainLuaExecuted
}
