use crate::{
    assets::map::tiled::SimpleWarfareMap,
    bevy_ext::app::AppExt,
    mod_engine::server::ModServer,
    statistics::{MapState, NetState, SelectedMap},
};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

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
    simple_warfare_maps: Res<Assets<SimpleWarfareMap>>,
    asset_server: Res<AssetServer>,
    selected_map: Res<SelectedMap>,
    mut mod_server: ResMut<ModServer>,
    mut net_state: ResMut<NextState<NetState>>,
    mut map_state: ResMut<NextState<MapState>>,
) {
    net_state.set(NetState::HostServer);
    map_state.set(MapState::CreatingMap);
    commands.spawn((
        TiledMap(
            asset_server.load(
                simple_warfare_maps
                    .get(selected_map.0.id())
                    .unwrap()
                    .map_path
                    .as_path(),
            ),
        ),
        TilemapAnchor::Center,
    ));
    mod_server.want_spawn_unit("example:Tank");
    mod_server.want_spawn_unit("钠锘聚核:Sunflower");
}
