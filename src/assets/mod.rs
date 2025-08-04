pub mod byte;
pub mod custom;
pub mod js_file;
pub mod map;
pub mod mods;
pub mod texture;

use std::path::Path;

use bevy::prelude::*;

use crate::{
    assets::{
        byte::{ByteFile, ByteFileLoader},
        custom::map::grid_layers::{CustomGridLayers, CustomGridLayersLoader},
        js_file::{JsTomlFile, JsTomlFileLoader},
        map::tiled::{
            SimpleWarfareMap, SimpleWarfareMapInfo, SimpleWarfareMapInfoLoader,
            SimpleWarfareMapLoader,
        },
        mods::{
            ModSet, ModSetLoader, ModSetNowUseConf, ModSetNowUseConfLoader, info::*, js::*, lua::*,
        },
        texture::{
            TextureAtlasLayoutHandles, chrome::ChromeTextureSlicer, dialog::DialogTextureSlicer,
            process_textures,
        },
    },
    consts::{MOD_SET_NOW_USE_CONF_PATH, MOD_SET_PATH},
    custom::{CustomModAsset, CustomModHandle},
    statistics::AppState,
};

// 宏用于快速生成资源结构体和默认实现
macro_rules! define_asset_group {
    ($name:ident<$asset_type:ident> { $($field:ident: $path:literal),* $(,)? }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $field: Handle<$asset_type>,)*
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $($field: Handle::<$asset_type>::default(),)*
                }
            }
        }

        impl $name {
            pub fn load(&mut self, asset_server: &Res<AssetServer>) {
                $(self.$field = asset_server.load($path);)*
            }
            pub fn all_untyped(&self) -> Vec<UntypedHandle>{
                vec![$(self.$field.clone().untyped()),*]
            }
        }
    };
}

define_asset_group!(Interfaces<Image>{
    loading_screen: "texture/interface/loading_screen.png",
    dialog:"texture/interface/dialog.png",
    chrome:"texture/interface/chrome.png",
    missing_map_thumbnail:"texture/interface/missing_map_thumbnail.png",
    too_larget_thumbnail:"texture/interface/too_larget_thumbnail.png",
});

#[derive(Debug, Default, Resource)]
pub struct GameAsset {
    pub interface: Interfaces,
    pub maps: Vec<Handle<SimpleWarfareMap>>,
    pub enable_mod_set: EnableModSet,
    pub custom_mod_handles: CustomModHandles,
    pub custom_mods: Option<Vec<CustomModAsset>>,
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
        app.init_asset::<JsTomlFile>() //js读取toml格式的文件将以String返回js
            .init_asset_loader::<JsTomlFileLoader>()
            .init_asset::<ModInfo>()
            .init_asset_loader::<ModInfoLoader>()
            .init_asset::<LuaAsset>()
            .init_asset_loader::<LuaAssetLoader>()
            .init_asset::<JsAsset>()
            .init_asset_loader::<JsAssetLoader>()
            .init_asset::<SimpleWarfareMap>()
            .init_asset_loader::<SimpleWarfareMapLoader>()
            .init_asset::<SimpleWarfareMapInfo>()
            .init_asset_loader::<SimpleWarfareMapInfoLoader>()
            .init_asset::<ModSetNowUseConf>()
            .init_asset_loader::<ModSetNowUseConfLoader>()
            .init_asset::<ModSet>()
            .init_asset_loader::<ModSetLoader>()
            .init_asset::<ByteFile>()
            .init_asset_loader::<ByteFileLoader>()
            .init_asset::<CustomGridLayers>()
            .init_asset_loader::<CustomGridLayersLoader>()
            .init_resource::<GameAsset>()
            .init_resource::<DialogTextureSlicer>()
            .init_resource::<ChromeTextureSlicer>()
            .init_resource::<TextureAtlasLayoutHandles>()
            .add_systems(OnEnter(AppState::AssetsLoading), load_assets)
            .add_systems(
                PreUpdate,
                check_assets_ready.run_if(in_state(AppState::AssetsLoading)),
            )
            .add_systems(OnEnter(AppState::AssetsProcessing), process_textures);
    }
}

fn load_assets(mut game_assets: ResMut<GameAsset>, asset_server: Res<AssetServer>) {
    game_assets.interface.load(&asset_server);
    let mod_set_conf_handle = asset_server.load(MOD_SET_NOW_USE_CONF_PATH);
    game_assets.enable_mod_set.conf_handle = mod_set_conf_handle.clone();

    // 收集所有资源句柄
    game_assets.assets_untyped_handle = game_assets.interface.all_untyped().to_vec();

    game_assets
        .assets_untyped_handle
        .push(mod_set_conf_handle.untyped());
}

fn check_assets_ready(
    mut game_asset: ResMut<GameAsset>,
    asset_server: Res<AssetServer>,
    mut system_state: ResMut<NextState<AppState>>,
    mod_set_confs: Res<Assets<ModSetNowUseConf>>,
) -> Result {
    game_asset
        .assets_untyped_handle
        .retain(|handle| !asset_server.is_loaded_with_dependencies(handle.id()));
    if game_asset.assets_untyped_handle.is_empty() {
        let mod_set_conf = mod_set_confs
            .get(game_asset.enable_mod_set.conf_handle.id())
            .ok_or(BevyError::from("Could not get the now_use.conf"))?;

        game_asset.enable_mod_set.mod_set_handle =
            asset_server.load(Path::new(MOD_SET_PATH).join(&mod_set_conf.use_mod_set));
        system_state.set(AppState::AssetsProcessing);
    }

    Ok(())
}
