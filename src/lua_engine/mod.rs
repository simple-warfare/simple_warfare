mod module;
use bevy::{asset::LoadedFolder, prelude::*};
use mlua::{Lua, ObjectLike, Table};

use crate::{
    app_state::AppState,
    assets::mods::{info::*, js::JsAsset, lua::*},
    mod_engine::server::ModServer,
};

#[derive(Resource)]
pub struct LuaRuntime {
    engine: Lua,
    global: Table,
}

impl Default for LuaRuntime {
    fn default() -> Self {
        let engine = Lua::new();
        let global = engine.globals();
        //添加默认module
        if let Ok(simple_warfare) = module::mod_engine(&engine) {
            global.set("simple_warfare", simple_warfare).expect("");
        }
        Self { engine, global }
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
    let engine = &lua_runtime.engine;
    for (mod_info_id, mod_info) in mod_infos.iter() {
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
            add_default_value(engine, global, mod_info).expect("add default value error");
            engine.load(lua_asset.context.clone()).exec()?;

            global.call_function::<()>("Main", ())?;

            let mod_enable_form_lua: ModEnableLua = global.get("mod_enable")?;

            let mod_enables: Vec<(JsAsset, Vec<String>)> = mod_enable_form_lua
                .enable
                .iter()
                .filter_map(|mod_class_lua| {
                    let js_handle = asset_server
                        .get_handle(
                            asset_server
                                .get_path(mod_info_id)?
                                .parent()?
                                .resolve(&mod_class_lua.js_file)
                                .ok()?,
                        )
                        .unwrap();
                    if let Some(js_asset) = js_assets.get(js_handle.id()) {
                        Some((js_asset.clone(), mod_class_lua.classes.clone()))
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

fn add_default_value(engine: &Lua, global: &Table, mod_info: &ModInfo) -> Result {
    let mod_info_lua = engine.create_ser_userdata(mod_info.clone())?;
    let mod_enable_lua = engine.create_ser_userdata(ModEnableLua::default())?;
    global.set("mod_info", mod_info_lua)?;
    global.set("mod_enable", mod_enable_lua)?;
    Ok(())
}
