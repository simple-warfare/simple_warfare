use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_northstar::prelude::*;

use crate::custom::map::navigator_layer::northstar::CustomGridLayersServer;

pub struct SimpleWarfareNorthStarPlugin;

impl Plugin for SimpleWarfareNorthStarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            NorthstarPlugin::<OrdinalNeighborhood>::default(),
            NorthstarDebugPlugin::<OrdinalNeighborhood>::default(),
        ))
        .add_observer(setup);
    }
}

fn setup(
    trigger: Trigger<TiledEvent<MapCreated>>,
    mut commands: Commands,
    map_assets: Res<Assets<TiledMapAsset>>,
    custom_grid_layers_server: Res<CustomGridLayersServer>,
) {

    let map = trigger.get_map_asset(&map_assets).unwrap();
    let tilemap_size = map.tilemap_size;

    let grid_settings = GridSettingsBuilder::new_2d(tilemap_size.x, tilemap_size.y)
        .chunk_size(16)
        .enable_collision()
        .build();
}
