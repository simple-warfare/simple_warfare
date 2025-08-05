use bevy::{log::tracing_subscriber::layer, prelude::*};
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
    let tiled_map_asset = trigger.get_map_asset(&map_assets).unwrap();
    let tiled_map = &tiled_map_asset.map;
    let map_layers = tiled_map
        .layers()
        .map(|layer| (layer.user_type.clone(), layer))
        .collect::<HashMap<_, _>>();
    let tilemap_size = tiled_map_asset.tilemap_size;

    info!("layers:{:?}", custom_grid_layers_server.layer);
    for grid_layer in custom_grid_layers_server.layer.iter() {
        let grid_settings = GridSettingsBuilder::new_2d(tilemap_size.x, tilemap_size.y)
            .chunk_size(16)
            .build();

        let mut grid = Grid::<OrdinalNeighborhood>::new(&grid_settings);

        for (user_type, custom_tile) in grid_layer.custom_tile.iter() {
            let Some(layer) = map_layers.get(&Some(user_type.clone())) else {
                continue;
            };

            let Some(tiles_layer) = layer.as_tile_layer() else {
                continue;
            };
            tiled_map_asset.for_each_tile(&tiles_layer, |layer_tile, _, tile_pos, _| {
                let Some(tile) = layer_tile.get_tile() else {
                    return;
                };

                if tile.user_type == Some(user_type.clone()) {
                    grid.set_nav(UVec3::new(tile_pos.x, tile_pos.y, 0), custom_tile.nav);
                }
            });
        }
    }
}
