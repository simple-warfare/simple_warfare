pub mod byte;
pub mod custom;
pub mod js_file;
pub mod map;
pub mod mods;
pub mod server;

use std::path::Path;

use bevy::prelude::*;

use crate::{
    consts::{MOD_SET_NOW_USE_CONF_PATH, MOD_SET_PATH},
    custom::{CustomModAsset, CustomModHandle},
    statistics::ServerState,
};

use self::{
    byte::{ByteFile, ByteFileLoader},
    custom::map::grid_layers::{CustomGridLayers, CustomGridLayersTomlLoader},
    js_file::{
        section::{SectionFile, SectionFileLoader},
        toml::{TomlFile, TomlFileLoader},
    },
    mods::{
        ModSet, ModSetJsonLoader, ModSetNowUseConf, ModSetNowUseConfLoader, ModSetTomlLoader,
        info::{ModInfo, ModInfoJsonLoader, ModInfoTomlLoader},
        js::{JsAsset, JsAssetLoader},
        lua::{LuaAsset, LuaAssetLoader},
    },
};

#[derive(Debug, Default, Resource)]
pub struct GameAsset {
    //pub maps: Vec<Handle<SimpleWarfareMap>>,
    pub enable_mod_set: EnableModSet,
    pub custom_mod_handles: CustomModHandles,
    pub assets_untyped_handle: Vec<UntypedHandle>,
}

#[derive(Debug, Default)]
pub struct CustomModHandles {
    pub mod_handles: Vec<CustomModHandle>,
    pub untyped_handles: Vec<UntypedHandle>,
}
#[derive(Debug, Default)]
pub struct EnableModSet {
    pub conf_handle: Handle<ModSetNowUseConf>,
    pub mod_set_handle: Handle<ModSet>,
}
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<TomlFile>() //js读取toml格式的文件将以String返回js
            .init_asset_loader::<TomlFileLoader>()
            .init_asset::<SectionFile>()
            .init_asset_loader::<SectionFileLoader>()
            .init_asset::<ModInfo>()
            .init_asset_loader::<ModInfoTomlLoader>()
            .init_asset_loader::<ModInfoJsonLoader>()
            .init_asset::<LuaAsset>()
            .init_asset_loader::<LuaAssetLoader>()
            .init_asset::<JsAsset>()
            .init_asset_loader::<JsAssetLoader>()
            .init_asset::<ModSetNowUseConf>()
            .init_asset_loader::<ModSetNowUseConfLoader>()
            .init_asset::<ModSet>()
            .init_asset_loader::<ModSetTomlLoader>()
            .init_asset_loader::<ModSetJsonLoader>()
            .init_asset::<ByteFile>()
            .init_asset_loader::<ByteFileLoader>()
            .init_asset::<CustomGridLayers>()
            .init_asset_loader::<CustomGridLayersTomlLoader>()
            .init_resource::<GameAsset>()
            .add_systems(OnEnter(ServerState::AssetsLoading), load_assets)
            .add_systems(
                PreUpdate,
                check_assets_ready.run_if(in_state(ServerState::AssetsLoading)),
            );
    }
}

fn load_assets(mut game_assets: ResMut<GameAsset>, asset_server: Res<AssetServer>) {
    info!("加载文件中....");
    let mod_set_conf_handle = asset_server.load(MOD_SET_NOW_USE_CONF_PATH);
    game_assets.enable_mod_set.conf_handle = mod_set_conf_handle.clone();

    game_assets
        .assets_untyped_handle
        .push(mod_set_conf_handle.untyped());
}

fn check_assets_ready(
    mut game_asset: ResMut<GameAsset>,
    asset_server: Res<AssetServer>,
    mut server_state: ResMut<NextState<ServerState>>,
    mod_set_confs: Res<Assets<ModSetNowUseConf>>,
) -> Result {
    game_asset
        .assets_untyped_handle
        .retain(|handle| !asset_server.is_loaded_with_dependencies(handle.id()));
    if game_asset.assets_untyped_handle.is_empty() {
        info!("文件加载完成");
        info!("处理文件中....");

        let mod_set_conf = mod_set_confs
            .get(game_asset.enable_mod_set.conf_handle.id())
            .ok_or(BevyError::from("Could not get the now_use.conf"))?;

        game_asset.enable_mod_set.mod_set_handle =
            asset_server.load(Path::new(MOD_SET_PATH).join(&mod_set_conf.use_mod_set));

        info!("处理文件中完成");
        info!("初始化 JsContext 中....");
        server_state.set(ServerState::JsContextInitiating);
    }

    Ok(())
}
