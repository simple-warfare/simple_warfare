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

fn setup(
    trigger: Trigger<TiledMapCreated>,
    mut commands: Commands,
    map_asset: Res<Assets<TiledMap>>,
) {
    info!("TiledMapCreated");

    let map = &map_asset.get(trigger.asset_id).unwrap().map;
    let tilemap_size = map_asset.get(trigger.asset_id).unwrap().tilemap_size;

    let grid_settings = GridSettingsBuilder::new_2d(tilemap_size.x, tilemap_size.y)
        .chunk_size(16)
        .enable_collision()
        .build();

    let mut grid = Grid::<OrdinalNeighborhood>::new(&grid_settings);

    for layer in map.layers() {
        if let Some(tile_layer) = layer.as_tile_layer() {
            let width = tile_layer.width().unwrap();
            let height = tile_layer.height().unwrap();

            for x in 0..width {
                for y in 0..height {
                    let tile = tile_layer.get_tile(x as i32, y as i32);
                    if let Some(tile) = tile {
                        if let Some(user_type) = tile.get_tile().unwrap().user_type.clone() {
                            if user_type == "water" {
                                grid.set_nav(UVec3::new(x, height - 1 - y, 0), Nav::Impassable);
                            }
                        }
                    }
                }
            }
        }
    }

    grid.build();

    commands.entity(trigger.entity).insert(grid);
}
