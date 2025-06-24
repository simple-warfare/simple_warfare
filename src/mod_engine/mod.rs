mod module;
pub mod r#struct;
use bevy::{asset::LoadedFolder, prelude::*};
use mlua::{Lua, ObjectLike};

use crate::{
    app_state::AppState,
    assets::mods::{info::*, lua::*},
    js_engine::event::{JsEngineEvent, ModEvent},
};

pub struct ModEnginePlugin;

impl Plugin for ModEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ModInfoLoading), load_mod_infos)
            .add_systems(
                Update,
                check_mod_infos.run_if(in_state(AppState::ModInfoLoading)),
            )
            .add_systems(OnEnter(AppState::ModInfoLoaded), load_main_lua)
            .add_systems(OnEnter(AppState::MainLuaExecuted), init_smilodon_engine);
    }
}

fn load_mod_infos(mut command: Commands, asset_server: Res<AssetServer>) {
    command.insert_resource(ModsFolderHandle(asset_server.load_folder("mods")));
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
    mut next_state: ResMut<NextState<AppState>>,
    mut js_engine_event_writer: EventWriter<JsEngineEvent>,
) -> Result {
    //获取lua环境

    next_state.set(AppState::MainLuaExecuting);
    let lua = get_lua()?;
    let global = lua.globals();
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
            let mod_info_lua = lua.create_ser_userdata(mod_info.clone())?;
            //添加该mod信息
            global.set("mod_info", mod_info_lua)?;
            lua.load(lua_asset.context.clone()).exec()?;

            global.call_function::<()>("Main", ())?;

            let mod_info_form_lua: ModInfo = global.get("mod_info")?;
            let code_js = r#"
            import { CustomUnit, Core } from "./simple_warfare_engine.js"

            class Tank extends CustomUnit {
                constructor() {
                    let core = new Core("坦克", 1000, 1000)
                    super(core)
                };
            };

            export{Tank};
            "#;
            js_engine_event_writer.write(JsEngineEvent::ModEvent(ModEvent::LoadJs(
                crate::assets::mods::js::JsAsset {
                    context: code_js.to_string(),
                },
            )));
        }
    }

    next_state.set(AppState::MainLuaExecuted);
    Ok(())
}

fn get_lua() -> Result<Lua> {
    let lua = Lua::new();
    let global = lua.globals();
    //添加默认module
    if let Ok(simple_warfare) = module::mod_engine(&lua) {
        global.set("simple_warfare", simple_warfare)?;
    }
    Ok(lua)
}

fn init_smilodon_engine() {}
