use std::{f32::consts::PI, ops::Deref};

use bevy::{
    color::palettes,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use bevy_ecs_tiled::prelude::*;
use rand::{Rng, rngs::ThreadRng};
use vleue_navigator2d::prelude::*;

use crate::helpers::navmesh_debug;

pub struct SimpleWarfareNavigatorPlugin;

impl Plugin for SimpleWarfareNavigatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VleueNavigatorPlugin,
            // Auto update the navmesh.
            // Obstacles will be entities with the `Obstacle` marker component,
            // and use the `Aabb` component as the obstacle data source.
            NavmeshUpdaterPlugin::<CachedObstacle<SharedShapeStorage>>::default(
            ),
        ))
        .add_observer(setup)
        .add_systems(Update, display_mesh)
        .add_systems(
            Startup,
            (
                navmesh_debug::setup_stats::<true>,
                navmesh_debug::setup_settings::<false>,
            ),
        )
        .add_systems(
            Update,
            (
                spawn_obstacle_on_click,
                navmesh_debug::update_stats::<SharedShapeStorage>,
                navmesh_debug::display_settings,
                navmesh_debug::update_settings::<10>,
            ),
        );
    }
}
fn setup(
    trigger: Trigger<TiledMapCreated>,
    mut commands: Commands,
    map_asset: Res<Assets<TiledMap>>,
) {
    let map = &map_asset.get(trigger.asset_id).unwrap().map;
    let tiled_width = map.tile_width as f32;
    let tiled_height = map.tile_height as f32;
    let tilemap_size = map_asset.get(trigger.asset_id).unwrap().tilemap_size;
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
            merge_steps: 4,
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

fn display_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    navmeshes: Res<Assets<NavMesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut current_mesh_entity: Local<Option<Entity>>,
    window_resized: EventReader<WindowResized>,
    navmesh: Single<(&ManagedNavMesh, Ref<NavMeshStatus>)>,
) {
    let (navmesh_handle, status) = navmesh.deref();
    if (!status.is_changed() || **status != NavMeshStatus::Built) && window_resized.is_empty() {
        return;
    }

    let Some(navmesh) = navmeshes.get(*navmesh_handle) else {
        return;
    };
    if let Some(entity) = *current_mesh_entity {
        commands.entity(entity).despawn();
    }

    *current_mesh_entity = Some(
        commands
            .spawn((
                Mesh2d(meshes.add(navmesh.to_mesh())),
                MeshMaterial2d(materials.add(ColorMaterial::from(
                    Color::Srgba(palettes::tailwind::BLUE_800).with_alpha(0.3),
                ))),
            ))
            .with_children(|main_mesh| {
                main_mesh.spawn((
                    Mesh2d(meshes.add(navmesh.to_wireframe_mesh())),
                    MeshMaterial2d(materials.add(ColorMaterial::from(Color::Srgba(
                        palettes::tailwind::TEAL_300,
                    )))),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                ));
            })
            .id(),
    );
}
const FACTOR: f32 = 7.0;
fn new_obstacle(commands: &mut Commands, rng: &mut ThreadRng, transform: Transform) -> Entity {
    commands
        .spawn((
            match rng.random_range(0..6) {
                0 => SharedShapeStorage::rectangle(
                    rng.random_range(1.0..5.0) * FACTOR,
                    rng.random_range(1.0..5.0) * FACTOR,
                ),
                1 => SharedShapeStorage::circle(rng.random_range(1.0..5.0) * FACTOR),
                2 => SharedShapeStorage::ellipse(
                    rng.random_range(1.0..5.0) * FACTOR,
                    rng.random_range(1.0..5.0) * FACTOR,
                ),
                3 => SharedShapeStorage::capsule(
                    rng.random_range(1.0..3.0) * FACTOR,
                    rng.random_range(1.5..5.0) * FACTOR,
                ),
                4 => SharedShapeStorage::round_rectangle(
                    rng.random_range(1.0..3.0) * FACTOR,
                    rng.random_range(1.5..5.0) * FACTOR,
                    rng.random_range(1.0..2.0) * FACTOR,
                ),
                5 => SharedShapeStorage::regular_polygon(
                    rng.random_range(1.0..5.0) * FACTOR,
                    rng.random_range(3..8),
                ),
                _ => unreachable!(),
            },
            transform,
        ))
        .id()
}

fn spawn_obstacle_on_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    mut settings: Single<&mut NavMeshSettings>,
) -> Result {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let Ok((camera, camera_transform)) = camera_q.single() else {
            return Ok(());
        };
        let window = *primary_window;
        if let Some(position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            let mut rng = rand::rng();
            let transform = Transform::from_translation(position.extend(0.0))
                .with_rotation(Quat::from_rotation_z(rng.random_range(0.0..(2.0 * PI))));
            settings
                .filter_obstacles
                .insert(new_obstacle(&mut commands, &mut rng, transform));
        }
    }
    Ok(())
}
