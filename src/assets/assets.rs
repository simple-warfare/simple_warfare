use bevy::prelude::*;

use crate::app_state::AppState;

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
    loadscreen: "interfaces/loading_screen.png",
    dialog:"interfaces/dialog.png",
});

#[derive(Debug, Default, Resource)]
pub struct GameAssets {
    pub interfaces: Interfaces,
    pub all_untyped_handles: Vec<UntypedHandle>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameAssets::default())
            .add_systems(
                OnEnter(AppState::AssetsLoading),
                |mut game_assets: ResMut<GameAssets>, asset_server: Res<AssetServer>| {
                    game_assets.interfaces.load(&asset_server);

                    // 收集所有资源句柄
                    game_assets.all_untyped_handles = game_assets
                        .interfaces
                        .all_untyped()
                        .iter()
                        .cloned()
                        .collect();
                },
            )
            .add_systems(
                PreUpdate,
                check_assets_ready.run_if(in_state(AppState::AssetsLoading)),
            );
    }
}

fn check_assets_ready(
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    mut system_state: ResMut<NextState<AppState>>,
) {
    // 检查所有资源是否都已加载完成
    let all_loaded = game_assets
        .all_untyped_handles
        .iter()
        .all(|handle| asset_server.is_loaded_with_dependencies(handle.id()));
    // 只有当所有资源都加载完成时才更新状态
    if all_loaded {
        system_state.set(AppState::AssetsProcessing);
    }
}
