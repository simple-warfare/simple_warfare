use std::sync::atomic::Ordering;

use aeronet::io::Session;
use bevy::prelude::*;
use simple_warfare_shared::prelude::{MessageDecode, MessageDecodeKind};

use crate::{
    adaptor::message::ServerMessage,
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
    mut mod_server: ResMut<ModServer>,
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
        server_state.set(ServerState::SomeAsyncWork);
    }
    Ok(())
}

fn check_some_async_works_completed(
    some_async_work_calculator: Res<SomeAsyncWorkCalculator>,
    mut server_state: ResMut<NextState<ServerState>>,
    mut session: Single<&mut Session, With<ChildOf>>,
    message_decode_kind: Res<MessageDecodeKind>,
) -> Result {
    if some_async_work_calculator.0.load(Ordering::Relaxed) == SOME_ASYNC_WORK_NUM {
        info!("SimpleWarfare启动完成");
        server_state.set(ServerState::Starting);

        session
            .send
            .push(ServerMessage::started_server().to_bytes(*message_decode_kind)?);
    }
    Ok(())
}
