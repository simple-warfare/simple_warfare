//! Avian physics backend for bevy_ecs_tiled.
//!
//! This module provides an implementation of the [`TiledPhysicsBackend`] trait using the Avian 2D physics engine.
//! This backend is only available when the `avian` feature is enabled.
//!
//! # Example
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_ecs_tiled::prelude::*;
//!
//! App::new()
//!     .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default());
//! ```
//!
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{
    ColliderCreated, MultiPolygon, TiledEvent, TiledPhysicsBackend, multi_polygon_as_line_strings,
    multi_polygon_as_triangles,
};
use parry2d::{
    math::{Isometry, Point, Real},
    shape::SharedShape,
};
use vleue_navigator2d::prelude::*;

/// The [`TiledPhysicsBackend`] to use for Parry 2D integration.
///
/// This enum allows you to select how colliders are generated from Tiled shapes:
/// - [`TiledPhysicsParryBackend::Polyline`]: Aggregates all line strings into a single polyline collider.
/// - [`TiledPhysicsParryBackend::Triangulation`]: Triangulates polygons and aggregates triangles into a compound collider.
/// - [`TiledPhysicsParryBackend::LineStrip`]: Creates a separate linestrip collider for each line string.
#[derive(Default, Reflect, Copy, Clone, Debug)]
#[reflect(Default, Debug)]
pub enum TiledPhysicsParryBackend {
    #[default]
    /// Aggregates all [`LineString`]s into a single collider using [`SharedShape::polyline`].
    Polyline,
    /// Performs triangulation and produces a single collider by aggregating multiple [`SharedShape::triangle`]s.
    Triangulation,
    /// Produces several linestrip colliders, one for each line string.
    LineStrip,
}

impl TiledPhysicsBackend for TiledPhysicsParryBackend {
    fn spawn_colliders(
        &self,
        commands: &mut Commands,
        _source: &TiledEvent<ColliderCreated>,
        multi_polygon: &MultiPolygon<f32>,
    ) -> Vec<Entity> {
        let mut out = vec![];
        match self {
            TiledPhysicsParryBackend::Triangulation => {
                let shared_shapes = multi_polygon_as_triangles(multi_polygon)
                    .iter()
                    .map(|([a, b, c], centroid)| {
                        (
                            Isometry::<Real>::new((*centroid).into(), 0.),
                            SharedShape::triangle((*a).into(), (*b).into(), (*c).into()),
                        )
                    })
                    .collect::<Vec<_>>();

                if !shared_shapes.is_empty() {
                    let shared_shape = SharedShape::compound(shared_shapes);
                    out.push(
                        commands
                            .spawn((
                                Name::from("Avian[Triangulation]"),
                                CachedObstacle::<SharedShapeStorage>::new(
                                    SharedShapeStorage::from(shared_shape),
                                ),
                                CachableObstacle,
                            ))
                            .id(),
                    );
                }
            }
            TiledPhysicsParryBackend::LineStrip => {
                multi_polygon_as_line_strings(multi_polygon)
                    .iter()
                    .enumerate()
                    .for_each(|(i, ls)| {
                        let shared_shape = SharedShape::polyline(
                            ls.points().map(|v| Point::new(v.x(), v.y())).collect(),
                            None,
                        );
                        out.push(
                            commands
                                .spawn((
                                    Name::from(format!("Avian[LineStrip {i}]")),
                                    CachedObstacle::<SharedShapeStorage>::new(
                                        SharedShapeStorage::from(shared_shape),
                                    ),
                                    CachableObstacle,
                                ))
                                .id(),
                        );
                    });
            }
            TiledPhysicsParryBackend::Polyline => {
                let mut vertices = vec![];
                let mut indices = vec![];
                multi_polygon_as_line_strings(multi_polygon)
                    .iter()
                    .for_each(|ls| {
                        ls.lines().for_each(|l| {
                            let points = l.points();
                            let len = vertices.len();
                            vertices.push(Point::new(points.0.x(), points.0.y()));
                            vertices.push(Point::new(points.1.x(), points.1.y()));
                            indices.push([len as u32, (len + 1) as u32]);
                        });
                    });
                if !vertices.is_empty() {
                    let shared_shape = SharedShape::polyline(vertices, Some(indices));
                    out.push(
                        commands
                            .spawn((
                                Name::from("Avian[Polyline]"),
                                CachedObstacle::<SharedShapeStorage>::new(
                                    SharedShapeStorage::from(shared_shape),
                                ),
                                CachableObstacle,
                            ))
                            .id(),
                    );
                }
            }
        }
        out
    }
}
