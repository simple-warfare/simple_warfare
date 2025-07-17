mod module;
pub mod user_data;
use bevy::{asset::LoadedFolder, prelude::*};
use mlua::{Lua, ObjectLike, Table};

use crate::{
    app_state::AppState,
    assets::mods::{info::*, js::JsAsset, lua::*},
    lua_engine::user_data::{MapManager, ModManager},
    mod_engine::server::ModServer,
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
            .add_systems(OnEnter(AppState::ModInfoLoading), load_mod_infos)
            .add_systems(
                Update,
                check_mod_infos.run_if(in_state(AppState::ModInfoLoading)),
            )
            .add_systems(OnEnter(AppState::ModInfoLoaded), load_main_lua);
    }
}

fn load_mod_infos(mut command: Commands, asset_server: Res<AssetServer>) {
    command.insert_resource(ModsFolderHandle(asset_server.load_folder("mods/custom")));
}
fn check_mod_infos(
    mut next_state: ResMut<NextState<AppState>>,
    mods_folder_handle: Res<ModsFolderHandle>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&mods_folder_handle.0) {
            next_state.set(AppState::ModInfoLoaded);
        }
    }
}

fn load_main_lua(
    lua_assets: Res<Assets<LuaAsset>>,
    asset_server: Res<AssetServer>,
    mod_infos: Res<Assets<ModInfo>>,
    js_assets: Res<Assets<JsAsset>>,
    mut next_state: ResMut<NextState<AppState>>,
    mod_server: Res<ModServer>,
    lua_runtime: Res<LuaRuntime>,
) -> Result {
    //获取lua环境

    next_state.set(AppState::MainLuaExecuting);
    let global = &lua_runtime.global;
    let context = &lua_runtime.context;
    for (mod_info_id, mod_info) in mod_infos
        .iter()
        .filter(|(_, info)| !info.game_version.is_empty())
    {
        let lua_handle = asset_server
            .get_handle(
                asset_server
                    .get_path(mod_info_id)
                    .unwrap()
                    .parent()
                    .unwrap()
                    .resolve("main.lua")?,
            )
            .unwrap();

        //载入main.lua
        if let Some(lua_asset) = lua_assets.get(lua_handle.id()) {
            //添加该mod信息
            add_global_value(context, global, mod_info).expect("add global value error");
            context.load(lua_asset.context.clone()).exec()?;

            global.call_function::<()>("Main", ())?;

            //mod初始化完毕
            let mod_manager: ModManager = global.get("mod_manager")?;

            let mod_enables: Vec<(JsAsset, Vec<String>)> = mod_manager
                .enables
                .iter()
                .filter_map(|mod_enable_classes_define| {
                    let js_handle = asset_server
                        .get_handle(
                            asset_server
                                .get_path(mod_info_id)?
                                .parent()?
                                .resolve(&mod_enable_classes_define.js_file_path)
                                .ok()?,
                        )
                        .unwrap();
                    if let Some(js_asset) = js_assets.get(js_handle.id()) {
                        Some((js_asset.clone(), mod_enable_classes_define.classes.clone()))
                    } else {
                        None
                    }
                })
                .collect();

            mod_server.load_mod(mod_enables, mod_info.clone())?;
        }
    }

    next_state.set(AppState::MainLuaExecuted);
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
