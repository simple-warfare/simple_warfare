mod module;
pub mod user_data;
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use mlua::{Lua, ObjectLike, Table};

use crate::{
    assets::{
        GameAsset,
        mods::{ModSet, info::*, lua::*},
    },
    consts::CUSTOM_MOD_PATH,
    custom::{
        CustomModEnableJsHandle, CustomModHandle,
        map::navigator_layer::northstar::CustomGridLayersServer,
    },
    lua_engine::user_data::{MapManager, ModManager, NavigatorLayerManager},
    statistics::ServerState,
};

#[derive(Resource)]
pub struct LuaRuntime {
    context: Lua,
    global: Table,
}

impl Default for LuaRuntime {
    fn default() -> Self {
        let context = Lua::new();
        let global = context.globals();
        //添加默认module
        if let Ok(simple_warfare) = module::mod_engine(&context) {
            global.set("simple_warfare", simple_warfare).expect("");
        }
        Self { context, global }
    }
}

pub struct LuaEnginePlugin;

impl Plugin for LuaEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LuaRuntime>()
            .add_systems(
                Update,
                check_mod_set.run_if(in_state(ServerState::ModSetLoading)),
            )
            .add_systems(
                Update,
                check_custom_mods.run_if(in_state(ServerState::CustomModLoading)),
            )
            .add_systems(OnEnter(ServerState::MainLuaExecuting), exec_mod_main_lua);
    }
}

fn check_mod_set(
    asset_server: Res<AssetServer>,
    mut game_asset: ResMut<GameAsset>,
    mut next_state: ResMut<NextState<ServerState>>,
    mod_sets: Res<Assets<ModSet>>,
) -> Result {
    let mod_set_id = game_asset.enable_mod_set.mod_set_handle.id();
    if asset_server.is_loaded_with_dependencies(mod_set_id) {
        let now_mod_set = mod_sets
            .get(mod_set_id)
            .ok_or(BevyError::from("Could not get the ModSet Asset"))?;
        now_mod_set.enable_mods.iter().for_each(|mod_name| {
            let info_handle = asset_server.load(
                Path::new(CUSTOM_MOD_PATH)
                    .join(mod_name)
                    .join("mod_info.toml"),
            );
            let main_lua_handle =
                asset_server.load(Path::new(CUSTOM_MOD_PATH).join(mod_name).join("main.lua"));

            game_asset
                .custom_mod_handles
                .untyped_handles
                .push(info_handle.clone().untyped());
            game_asset
                .custom_mod_handles
                .untyped_handles
                .push(main_lua_handle.clone().untyped());

            game_asset
                .custom_mod_handles
                .mod_handles
                .push(CustomModHandle::new(info_handle, main_lua_handle));
        });
        info!("CustomMod Loading");
        next_state.set(ServerState::CustomModLoading);
    }

    Ok(())
}

fn check_custom_mods(
    mut game_asset: ResMut<GameAsset>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<ServerState>>,
) {
    game_asset
        .custom_mod_handles
        .untyped_handles
        .retain(|handle| !asset_server.is_loaded_with_dependencies(handle.id()));

    if game_asset.custom_mod_handles.untyped_handles.is_empty() {
        info!("MainLua Executing");
        next_state.set(ServerState::MainLuaExecuting);
    }
}

fn exec_mod_main_lua(
    asset_server: Res<AssetServer>,
    lua_assets: Res<Assets<LuaAsset>>,
    mod_infos: Res<Assets<ModInfo>>,
    mut game_asset: ResMut<GameAsset>,
    mut next_state: ResMut<NextState<ServerState>>,
    lua_runtime: Res<LuaRuntime>,
    mut custom_grid_layers_server: ResMut<CustomGridLayersServer>,
) -> Result {
    //获取lua环境
    let global = &lua_runtime.global;
    let context = &lua_runtime.context;

    let custom_mods = &mut game_asset.custom_mod_handles;
    custom_mods
        .mod_handles
        .iter_mut()
        .try_for_each::<_, Result>(|custom_mod| {
            if let (Some(lua_asset), Some(mod_info)) = (
                lua_assets.get(custom_mod.main_lua.id()),
                mod_infos.get(custom_mod.info.id()),
            ) {
                let mod_name = &mod_info.name;

                add_global_value(context, global, mod_info).expect("add global value error");
                context.load(lua_asset.context.clone()).exec()?;

                global.call_function::<()>("Main", ())?;

                //mod初始化完毕
                let mod_manager = global.get::<ModManager>(ModManager::LUA_GLOBAL_NAME)?;

                mod_manager
                    .enables
                    .iter()
                    .for_each(|mod_enable_classes_define| {
                        let js_handle = asset_server.load(
                            Path::new(CUSTOM_MOD_PATH)
                                .join(mod_name)
                                .join(&mod_enable_classes_define.js_file_path),
                        );

                        custom_mod
                            .custom_mod_enable_js_handles
                            .push(CustomModEnableJsHandle::new(
                                js_handle.clone(),
                                mod_enable_classes_define.classes.clone(),
                            ));

                        custom_mods.untyped_handles.push(js_handle.untyped());
                    });

                let map_manager = global.get::<MapManager>(MapManager::LUA_GLOBAL_NAME)?;
                // map_manager.map_paths.iter().try_for_each(|map_path| {
                //     let binding = get_real_path(mod_name, map_path);
                //     let real_map_path = binding.as_path();

                //     if let Some(ext) = real_map_path.extension() {
                //         let (map, untyped) = match ext.to_string_lossy().trim() {
                //             "tmx" => {
                //                 let map = asset_server.load(real_map_path);
                //                 (map.clone(), map.untyped())
                //             }
                //             _ => return Err(BevyError::from("undefine map type")),
                //         };
                //         custom_mod.maps.push(map);
                //         custom_mods.untyped_handles.push(untyped);
                //     }
                //     Ok(())
                // })?;

                let navigator_layer_manager =
                    global.get::<NavigatorLayerManager>(NavigatorLayerManager::LUA_GLOBAL_NAME)?;
                navigator_layer_manager.layers_path.iter().for_each(|path| {
                    custom_grid_layers_server.new_layer(get_real_path(mod_name, path))
                });
            }
            Ok(())
        })?;
        info!("JsFile Loading");
    next_state.set(ServerState::JsFileLoading);
    Ok(())
}

fn add_global_value(context: &Lua, global: &Table, mod_info: &ModInfo) -> Result {
    let mod_info = context.create_ser_userdata(mod_info.clone())?;
    let mod_manager = context.create_ser_userdata(ModManager::default())?;
    let map_manager = context.create_ser_userdata(MapManager::default())?;
    let navigator_layer_manager = context.create_ser_userdata(NavigatorLayerManager::default())?;
    global.set("mod_info", mod_info)?;
    global.set(ModManager::LUA_GLOBAL_NAME, mod_manager)?;
    global.set(MapManager::LUA_GLOBAL_NAME, map_manager)?;
    global.set(
        NavigatorLayerManager::LUA_GLOBAL_NAME,
        navigator_layer_manager,
    )?;
    Ok(())
}

pub fn get_real_path<P>(mod_name: P, small_path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    Path::new(CUSTOM_MOD_PATH).join(mod_name).join(small_path)
}
