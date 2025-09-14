use std::{path::PathBuf, sync::Arc};

use bevy::{
    asset::uuid::Uuid,
    platform::collections::{HashMap, HashSet},
    prelude::*,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{assets::custom::map::grid_layers::CustomGridLayers, custom::map::CustomTile};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CustomGridLayer {
    pub uuid: Uuid,
    pub movement_type: String,
    pub merge_with: HashSet<Uuid>,
    pub custom_tile: Vec<CustomTile>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CustomGridLayerStorage {
    pub movement_type: String,
    pub merge_with: HashSet<Uuid>,
    pub custom_tile: HashMap<String, CustomTile>,
}

impl CustomGridLayer {
    pub fn storage(&self) -> CustomGridLayerStorage {
        CustomGridLayerStorage {
            movement_type: self.movement_type.clone(),
            merge_with: self.merge_with.clone(),
            custom_tile: self
                .custom_tile
                .iter()
                .map(|tile| (tile.user_type.clone(), tile.clone()))
                .collect(),
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct CustomGridLayersServer {
    pub new_layer: Vec<PathBuf>,
    pub handles: Option<Vec<Handle<CustomGridLayers>>>,
    pub layer: IndexMap<CustomGridLayersIndex, CustomGridLayerStorage>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct CustomGridLayersIndex {
    pub movement_type: String,
    pub merge_with: Vec<Uuid>,
}

impl CustomGridLayersIndex {
    pub fn new(movement_type: String, merge_with: Vec<Uuid>) -> Self {
        Self {
            movement_type,
            merge_with,
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct NorthstarGridEntitiesStorage(pub HashMap<Arc<Vec<Uuid>>, Entity>);

impl CustomGridLayersServer {
    pub fn new_layer(&mut self, path: PathBuf) {
        self.new_layer.push(path);
    }
    pub fn add_layer(&mut self, layers: CustomGridLayers) {
        for layer in &layers.layer {
            if let Some(index) = self.find_matching_layer_index(layer) {
                self.merge_layer_at_index(index, layer);
            } else {
                self.layer.insert(
                    CustomGridLayersIndex::new(
                        layer.movement_type.clone(),
                        layer.merge_with.iter().map(|m| *m).collect(),
                    ),
                    layer.storage(),
                );
            }
        }
    }

    fn find_matching_layer_index(&self, layer: &CustomGridLayer) -> Option<usize> {
        if self.layer.is_empty() {
            return None;
        }

        self.layer.iter().find_map(|(merge_with, storage_layer)| {
            if storage_layer.movement_type == layer.movement_type
                && storage_layer
                    .merge_with
                    .iter()
                    .any(|m| layer.merge_with.contains(m))
            {
                Some(self.layer.get_index_of(merge_with).unwrap())
            } else {
                None
            }
        })
    }

    fn merge_layer_at_index(&mut self, index: usize, layer: &CustomGridLayer) {
        let updated_layer = &mut self.layer[index];

        updated_layer
            .merge_with
            .extend(layer.merge_with.iter().cloned());

        for tile in &layer.custom_tile {
            updated_layer
                .custom_tile
                .insert(tile.user_type.clone(), tile.clone());
        }
    }
}
