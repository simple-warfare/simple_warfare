use std::sync::{Arc, atomic::Ordering};

use bevy::prelude::*;
use bevy_fly_camera::FlyCamera2d;

use super::{Scene, SceneState};
use crate::{
    assets::{
        GameAsset,
        mods::{info::ModInfo, js::JsAsset, lua::LuaAsset},
    },
    bevy_ext::app::AppExt,
    mod_engine::server::ModServer,
    statistics::{AppState, SOME_ASYNC_WORK_NUM, SomeAsyncWorkCalculator},
};
use bevy_seedling::prelude::*;

#[derive(Default)]
pub struct LoadingScene;

#[derive(Component)]
struct LoadingSceneMark;

impl Scene for LoadingScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<LoadingSceneMark, _, _>(SceneState::LoadingScene, setup)
            .add_systems(
                Update,
                check_js_and_map.run_if(in_state(AppState::JsLoading)),
            )
            .add_systems(
                Update,
                check_some_async_works_completed.run_if(in_state(AppState::SomeAsyncWork)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game_asset: Res<GameAsset>) {
    commands.spawn((Camera2d, FlyCamera2d::default(), Camera::default()));
    let mut background_music = SamplePlayer::new(asset_server.load("music/background/war.mp3"));
    background_music.repeat_mode = RepeatMode::RepeatEndlessly;
    commands.spawn(background_music);
    commands.spawn((
        LoadingSceneMark,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_self: JustifySelf::Center,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        BackgroundColor(Color::BLACK),
        children![(
            Node {
                height: Val::Percent(150.),
                ..Default::default()
            },
            ImageNode {
                image: game_asset.interface.loading_screen.clone(),
                ..Default::default()
            },
        )],
    ));
}

fn check_js_and_map(
    mut game_asset: ResMut<GameAsset>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>,
    mod_server: Res<ModServer>,
    js_assets: Res<Assets<JsAsset>>,
    lua_assets: Res<Assets<LuaAsset>>,
    mod_infos: Res<Assets<ModInfo>>,
) -> Result {
    game_asset
        .custom_mod_handles
        .untyped_handles
        .retain(|handle| !asset_server.is_loaded_with_dependencies(handle.id()));

    if game_asset.custom_mod_handles.untyped_handles.is_empty() {
        app_state.set(AppState::ModLoading);
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
        let mut all_maps = game_asset
            .custom_mod_handles
            .mod_handles
            .iter()
            .flat_map(|custom_mod| custom_mod.maps.iter().cloned().map(Arc::new))
            .collect();
        game_asset.maps.append(&mut all_maps);

        app_state.set(AppState::SomeAsyncWork);
        //scene_state.set(SceneState::MainScene);
    }
    Ok(())
}

fn check_some_async_works_completed(
    some_async_work_calculator: Res<SomeAsyncWorkCalculator>,
    mut app_state: ResMut<NextState<AppState>>,
    mut scene_state: ResMut<NextState<SceneState>>,
) {
    if some_async_work_calculator.0.load(Ordering::Relaxed) == SOME_ASYNC_WORK_NUM {
        app_state.set(AppState::AllReady);
        scene_state.set(SceneState::MainScene);
    }
}
