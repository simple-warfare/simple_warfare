use std::sync::Arc;

use bevy::prelude::*;

use super::{Scene, SceneState};
use crate::{
    app_state::AppState,
    assets::{
        GameAsset,
        mods::{info::ModInfo, js::JsAsset},
    },
    bevy_ext::app::AppExt,
    mod_engine::server::ModServer,
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
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game_asset: Res<GameAsset>) {
    commands.spawn((Camera2d, Camera::default()));
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
    mut scene_state: ResMut<NextState<SceneState>>,
    mod_server: Res<ModServer>,
    js_assets: Res<Assets<JsAsset>>,
    mod_infos: Res<Assets<ModInfo>>,
) -> Result {
    game_asset
        .custom_mods
        .untyped_handles
        .retain(|handle| !asset_server.is_loaded_with_dependencies(handle.id()));

    if game_asset.custom_mods.untyped_handles.is_empty() {
        app_state.set(AppState::ModLoading);
        game_asset
            .custom_mods
            .mods
            .iter()
            .try_for_each::<_, Result>(|custom_mod| {
                if let Some(info) = mod_infos.get(custom_mod.info.id()) {
                    mod_server.load_mod(
                        custom_mod
                            .enable_js
                            .iter()
                            .filter_map(|(js_handle, enables)| {
                                if let Some(js_asset) = js_assets.get(js_handle.id()) {
                                    Some((js_asset.clone(), enables.clone()))
                                } else {
                                    None
                                }
                            })
                            .collect(),
                        info.clone(),
                    )
                } else {
                    Err(BevyError::from("Could not get the info asset"))
                }
            })?;
        let mut all_maps = game_asset
            .custom_mods
            .mods
            .iter()
            .flat_map(|custom_mod| custom_mod.maps.iter().cloned().map(|map| Arc::new(map)))
            .collect();
        game_asset.maps.append(&mut all_maps);

        app_state.set(AppState::ModLoaded);
        scene_state.set(SceneState::MainScene);
    }
    Ok(())
}
