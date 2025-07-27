use std::ops::Deref;

use bevy::{color::palettes, prelude::*, window::WindowResized};
use vleue_navigator::prelude::*;

pub struct SimpleWarfareNavigatorPlugin;

impl Plugin for SimpleWarfareNavigatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VleueNavigatorPlugin,
            // Auto update the navmesh.
            // Obstacles will be entities with the `Obstacle` marker component,
            // and use the `Aabb` component as the obstacle data source.
            NavmeshUpdaterPlugin::<SharedShapeStorage>::default(),
        ))
        .add_systems(Update, display_mesh);
    }
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
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::Srgba(
                    palettes::tailwind::BLUE_800,
                )))),
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
