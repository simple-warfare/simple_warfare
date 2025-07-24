mod module;
pub mod user_data;
use std::path::Path;

use bevy::prelude::*;
use mlua::{Lua, ObjectLike, Table};

use crate::{
    app_state::AppState,
    assets::{
        GameAsset,
        map::SimpleWarfareMap,
        mods::{ModSet, info::*, lua::*},
    },
    consts::CUSTOM_MOD_PATH,
    custom::{CustomModEnableJsHandle, CustomModHandle},
    lua_engine::user_data::{MapManager, ModManager},
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
                check_mod_set.run_if(in_state(AppState::ModSetLoading)),
            )
            .add_systems(
                Update,
                check_custom_mods.run_if(in_state(AppState::CustomModLoading)),
            )
            .add_systems(OnEnter(AppState::MainLuaExecuting), exec_mod_main_lua);
    }
}

fn check_mod_set(
    asset_server: Res<AssetServer>,
    mut game_asset: ResMut<GameAsset>,
    mut next_state: ResMut<NextState<AppState>>,
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
        next_state.set(AppState::CustomModLoading);
    }

    Ok(())
}

fn check_custom_mods(
    mut game_asset: ResMut<GameAsset>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    game_asset
        .custom_mod_handles
        .untyped_handles
        .retain(|handle| !asset_server.is_loaded_with_dependencies(handle.id()));

    if game_asset.custom_mod_handles.untyped_handles.is_empty() {
        next_state.set(AppState::MainLuaExecuting);
    }
}

fn exec_mod_main_lua(
    asset_server: Res<AssetServer>,
    lua_assets: Res<Assets<LuaAsset>>,
    mod_infos: Res<Assets<ModInfo>>,
    mut game_asset: ResMut<GameAsset>,
    mut next_state: ResMut<NextState<AppState>>,
    lua_runtime: Res<LuaRuntime>,
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
                let mod_manager = global.get::<ModManager>("mod_manager")?;

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

                let map_manager = global.get::<MapManager>("map_manager")?;
                map_manager.map_paths.iter().try_for_each(|map_path| {
                    let binding = Path::new(CUSTOM_MOD_PATH).join(mod_name).join(map_path);
                    let real_map_path = binding.as_path();

                    if let Some(ext) = real_map_path.extension() {
                        let (map, untyped) = match ext.to_string_lossy().trim() {
                            "tmx" => {
                                let map = asset_server.load(real_map_path);
                                (SimpleWarfareMap::Tiled(map.clone()), map.untyped())
                            }
                            "ldtk" => {
                                let map = asset_server.load(real_map_path);
                                (SimpleWarfareMap::Ldtk(map.clone()), map.untyped())
                            }
                            _ => return Err(BevyError::from("undefine map type")),
                        };
                        custom_mod.maps.push(map);
                        custom_mods.untyped_handles.push(untyped);
                    }
                    Ok(())
                })?;
            }
            Ok(())
        })?;
    next_state.set(AppState::JsLoading);
    Ok(())
}

fn add_global_value(context: &Lua, global: &Table, mod_info: &ModInfo) -> Result {
    let mod_info = context.create_ser_userdata(mod_info.clone())?;
    let mod_manager = context.create_ser_userdata(ModManager::default())?;
    let map_manager = context.create_ser_userdata(MapManager::default())?;
    global.set("mod_info", mod_info)?;
    global.set("mod_manager", mod_manager)?;
    global.set("map_manager", map_manager)?;
    Ok(())
}
