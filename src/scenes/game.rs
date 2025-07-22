use crate::{
    assets::map::{ldtk::LdtkMap, tiled::TiledMap},
    bevy_ext::app::AppExt,
    mod_engine::server::ModServer,
    statistics::{NetState, SelectedMap},
};
use bevy::prelude::*;
use bevy_ecs_tiled::{map::TiledMapHandle, prelude::TilemapAnchor};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct GameScene;

#[derive(Component)]
struct GameSceneMark;

impl Scene for GameScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<GameSceneMark, _, _>(SceneState::GameScene, setup);
    }
}

fn setup(
    mut commands: Commands,
    ldtk_maps: Res<Assets<LdtkMap>>,
    tiled_maps: Res<Assets<TiledMap>>,
    asset_server: Res<AssetServer>,
    selected_map: Res<SelectedMap>,
    mod_server: Res<ModServer>,
    mut net_state: ResMut<NextState<NetState>>,
) {
    net_state.set(NetState::HostServer);
    commands.spawn((
        TiledMapHandle(asset_server.load(selected_map.0.get_path(&tiled_maps, &ldtk_maps))),
        TilemapAnchor::Center,
    ));
    mod_server.spawn_unit("example:Tank");
    mod_server.spawn_unit("example:Tank");
    mod_server.spawn_unit("example:Tank");
}
