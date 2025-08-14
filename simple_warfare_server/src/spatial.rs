use std::time::Duration;

use bevy::prelude::*;
use bevy_spatial::{AutomaticUpdate, SpatialStructure, kdtree::KDTree2};

#[derive(Component, Default)]
pub struct Spatial;
pub type SpatialTree = KDTree2<Spatial>;
pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            AutomaticUpdate::<Spatial>::new()
                .with_spatial_ds(SpatialStructure::KDTree2)
                .with_frequency(Duration::from_millis(1)),
        );
    }
}
