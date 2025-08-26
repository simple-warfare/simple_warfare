use std::sync::atomic::Ordering;

use bevy::prelude::*;

use crate::{
    assets::{
        GameAsset,
        mods::{info::ModInfo, js::JsAsset, lua::LuaAsset},
    },
    mod_engine::server::ModServer,
    statistics::{SOME_ASYNC_WORK_NUM, ServerState, SomeAsyncWorkCalculator},
};

pub struct LoadingStatePlugin;

impl Plugin for LoadingStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_js_and_map.run_if(in_state(ServerState::JsFileLoading)),
        )
        .add_systems(
            Update,
            check_some_async_works_completed.run_if(in_state(ServerState::SomeAsyncWork)),
        );
    }
}

fn check_js_and_map(
    mut game_asset: ResMut<GameAsset>,
    asset_server: Res<AssetServer>,
    mut server_state: ResMut<NextState<ServerState>>,
    mod_server: Res<ModServer>,
    js_assets: Res<Assets<JsAsset>>,
    lua_assets: Res<Assets<LuaAsset>>,
    mod_infos: Res<Assets<ModInfo>>,
) -> Result {
    game_asset
        .custom_mod_handles
        .untyped_handles
        .retain(|handle| !asset_server.is_loaded_with_dependencies(handle.id()));

    if game_asset.custom_mod_handles.untyped_handles.is_empty() {
        game_asset
            .custom_mod_handles
            .mod_handles
            .iter()
            .map(|custom_mod_handle| {
                custom_mod_handle.to_asset(&js_assets, &mod_infos, &lua_assets)
            })
            .try_for_each::<_, Result>(|custom_mod_asset| {
                mod_server.load_mod(custom_mod_asset)?;
                Ok(())
            })?;

        // let mut all_maps: Vec<_> = game_asset
        //     .custom_mod_handles
        //     .mod_handles
        //     .iter()
        //     .flat_map(|custom_mod| custom_mod.maps.iter())
        //     .cloned()
        //     .collect();

        //game_asset.maps.append(&mut all_maps);
        info!("SomeAsyncWork");
        server_state.set(ServerState::SomeAsyncWork);
        //scene_state.set(SceneState::MainScene);
    }
    Ok(())
}

fn check_some_async_works_completed(
    some_async_work_calculator: Res<SomeAsyncWorkCalculator>,
    mut server_state: ResMut<NextState<ServerState>>,
) {
    if some_async_work_calculator.0.load(Ordering::Relaxed) == SOME_ASYNC_WORK_NUM {
        info!("Waiting");
        server_state.set(ServerState::Waiting);
    }
}
