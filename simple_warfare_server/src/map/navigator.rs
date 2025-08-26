use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use vleue_navigator::prelude::*;


pub struct SimpleWarfareNavigatorPlugin;

impl Plugin for SimpleWarfareNavigatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VleueNavigatorPlugin,
            // Auto update the navmesh.
            // Obstacles will be entities with the `Obstacle` marker component,
            // and use the `Aabb` component as the obstacle data source.
            NavmeshUpdaterPlugin::<CachedObstacle<SharedShapeObstacle>>::default(),
        ))
        .add_observer(setup);
    }
}
fn setup(
    trigger: Trigger<TiledEvent<MapCreated>>,
    mut commands: Commands,
    map_assets: Res<Assets<TiledMapAsset>>,
) {
    let map_asset = trigger.get_map_asset(&map_assets).unwrap();
    let tiled_width = map_asset.map.tile_width as f32;
    let tiled_height = map_asset.map.tile_height as f32;
    let tilemap_size = map_asset.tilemap_size;
    // Spawn a new navmesh that will be automatically updated.
    commands.spawn((
        NavMeshSettings {
            // Define the outer borders of the navmesh.
            // This will be in navmesh coordinates
            fixed: Triangulation::from_outer_edges(&[
                vec2(0.0, 0.0),
                vec2(tilemap_size.x as f32 * tiled_width, 0.0),
                vec2(
                    tilemap_size.x as f32 * tiled_width,
                    tilemap_size.y as f32 * tiled_height,
                ),
                vec2(0.0, tilemap_size.y as f32 * tiled_height),
            ]),
            // Starting with a small mesh simplification factor to avoid very small geometry.
            // Small geometry can make navmesh generation fail due to rounding errors.
            // This example has round obstacles which can create small details.
            simplify: 0.2,
            merge_steps: 2,
            ..default()
        },
        // Mark it for update as soon as obstacles are changed.
        // Other modes can be debounced or manually triggered.
        NavMeshUpdateMode::Direct,
        // This transform places the (0, 0) point of the navmesh, and is used to transform coordinates from the world to the navmesh.
        Transform::from_translation(Vec3::new(
            -(tilemap_size.x as f32 * tiled_width) / 2.0,
            -(tilemap_size.y as f32 * tiled_height) / 2.0,
            0.0,
        )),
    ));
}
