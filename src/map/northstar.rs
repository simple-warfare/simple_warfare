use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_northstar::prelude::*;

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

fn setup(trigger: Trigger<TiledMapCreated>, map_asset: Res<Assets<TiledMap>>) {
    info!("TiledMapCreated");

    let map = &map_asset.get(trigger.asset_id).unwrap().map;
    let tiled_width = map.tile_width;
    let tiled_height = map.tile_height;
    let tilemap_size = map_asset.get(trigger.asset_id).unwrap().tilemap_size;

    let grid_settings =
        GridSettingsBuilder::new_2d(tilemap_size.x * tiled_width, tilemap_size.y * tiled_height)
            .chunk_size(20)
            .enable_collision()
            .build();

    let mut grid = Grid::<OrdinalNeighborhood>::new(&grid_settings);
    grid.build();
}
