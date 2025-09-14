pub mod graphic;
pub mod light2d;
pub mod physics;
pub mod section;
pub mod transform;
pub mod turret;
pub mod data;
pub mod way_point;
pub mod animation;
pub mod commpont;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use bevy::prelude::*;

use crate::custom::unit::way_point::WayPointQueue;

#[derive(Event)]
pub struct NewSpawnedUnit(pub Entity);

#[derive(Debug, Component, Clone)]
pub struct CustomInnerInfo {
    pub module_parent_path: String,
    pub module_path: String,
}

#[derive(Debug, Component, Clone)]
pub struct CustomInnerInfoStorage {
    pub inner: Arc<CustomInnerInfo>,
}

impl CustomInnerInfoStorage {
    pub fn new(inner: Arc<CustomInnerInfo>) -> Self {
        Self { inner }
    }
}

impl CustomInnerInfo {
    pub fn new(module_path: impl Into<String>) -> Self {
        let module_path = module_path.into();
        Self {
            module_parent_path: Path::new(&module_path)
                .parent()
                .unwrap()
                .to_string_lossy()
                .into(),
            module_path,
        }
    }

    pub fn get_real_path<P: AsRef<Path>>(&self, relative_path: P) -> PathBuf {
        Path::new(&self.module_parent_path).join(relative_path)
    }
}

pub struct CustomUnitPlugin;

impl Plugin for CustomUnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WayPointQueue>();
    }
}
