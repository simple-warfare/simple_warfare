use bevy::prelude::*;

#[derive(States, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Reflect)]
pub enum AppState {
    #[default]
    AssetsLoading,
    AssetsProcessing,
    InitJsContext,
    ModSetLoading,
    CustomModLoading,
    MainLuaExecuting,
    MainLuaExecuted,
}
