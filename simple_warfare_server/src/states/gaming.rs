use crate::{
    mod_engine::server::ModServer,
    statistics::{MapBackendState, NetState},
};
use bevy::prelude::*;


#[derive(Default)]
pub struct GameScene;

impl Plugin for GameScene {
    fn build(&self, app: &mut App) {
    }
}

fn setup(
    mut commands: Commands,
    //simple_warfare_maps: Res<Assets<SimpleWarfareMap>>,
    asset_server: Res<AssetServer>,
    mut mod_server: ResMut<ModServer>,
    mut net_state: ResMut<NextState<NetState>>,
    mut map_state: ResMut<NextState<MapBackendState>>,
) {
    net_state.set(NetState::HostServer);
    map_state.set(MapBackendState::CreatingMap);
    mod_server.spawn_unit("钠锘聚核:Sunflower");
}
