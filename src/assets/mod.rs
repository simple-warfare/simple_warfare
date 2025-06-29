pub mod map;
pub mod mods;
pub mod texture;
use bevy::prelude::*;

use crate::{
    app_state::AppState,
    assets::{
        map::ldtk::{LdtkMap, LdtkMapLoader},
        mods::{info::*, js::*, lua::*},
        texture::{
            TextureAtlasLayoutHandles,
            interface::{ChromeTextureSlicer, DialogTextureSlicer},
            process_textures,
        },
    },
};

// 宏用于快速生成资源结构体和默认实现
macro_rules! define_asset_group {
    ($name:ident<$asset_type:ident> { $($field:ident: $path:literal),* $(,)? }) => {
        #[derive(Debug)]
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
});

#[derive(Debug, Default, Resource)]
pub struct GameAsset {
    pub interface: Interfaces,
    pub all_untyped_handle: Vec<UntypedHandle>,
}
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ModInfo>()
            .init_asset_loader::<ModInfoLoader>()
            .init_asset::<LuaAsset>()
            .init_asset_loader::<LuaAssetLoader>()
            .init_asset::<JsAsset>()
            .init_asset_loader::<JsAssetLoader>()
            .init_asset::<LdtkMap>()
            .init_asset_loader::<LdtkMapLoader>()
            .init_resource::<GameAsset>()
            .init_resource::<DialogTextureSlicer>()
            .init_resource::<ChromeTextureSlicer>()
            .init_resource::<TextureAtlasLayoutHandles>()
            .add_systems(
                OnEnter(AppState::AssetsLoading),
                |mut game_assets: ResMut<GameAsset>, asset_server: Res<AssetServer>| {
                    game_assets.interface.load(&asset_server);

                    // 收集所有资源句柄
                    game_assets.all_untyped_handle = game_assets
                        .interface
                        .all_untyped()
                        .iter()
                        .cloned()
                        .collect();
                },
            )
            .add_systems(
                PreUpdate,
                check_assets_ready.run_if(in_state(AppState::AssetsLoading)),
            )
            .add_systems(OnEnter(AppState::AssetsProcessing), process_textures);
    }
}

fn check_assets_ready(
    game_asset: Res<GameAsset>,
    asset_server: Res<AssetServer>,
    mut system_state: ResMut<NextState<AppState>>,
) {
    // 检查所有资源是否都已加载完成
    let all_loaded = game_asset
        .all_untyped_handle
        .iter()
        .all(|handle| asset_server.is_loaded_with_dependencies(handle.id()));
    // 只有当所有资源都加载完成时才更新状态
    if all_loaded {
        system_state.set(AppState::AssetsProcessing);
    }
}
