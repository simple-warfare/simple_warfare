use std::path::PathBuf;

use bevy::{asset::uuid::Uuid, platform::collections::HashSet, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{assets::custom::map::grid_layers::CustomGridLayers, custom::map::CustomTile};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CustomGridLayer {
    pub movement_type: String,
    pub merge_with: HashSet<Uuid>,
    pub custom_tile: Vec<CustomTile>,
}

#[derive(Debug, Default, Resource)]
pub struct CustomGridLayersServer {
    pub new_layer: Vec<PathBuf>,
    pub handles: Option<Vec<Handle<CustomGridLayers>>>,
    pub layer: Vec<CustomGridLayer>,
}

impl CustomGridLayersServer {
    pub fn new_layer(&mut self, path: PathBuf) {
        self.new_layer.push(path);
    }
    pub fn add_layer(&mut self, layers: CustomGridLayers) {
        for layer in &layers.layer {
            if self.layer.is_empty() {
                self.layer.push(layer.clone());
                continue;
            }

            let mut found = false;

            for (index, already_layer) in self.layer.clone().into_iter().enumerate() {
                if already_layer.movement_type == layer.movement_type {
                    if already_layer
                        .merge_with
                        .iter()
                        .any(|merge_with| layer.merge_with.contains(merge_with))
                    {
                        let mut updated_layer = already_layer.clone();
                        updated_layer
                            .merge_with
                            .extend(layer.merge_with.iter().cloned());
                        updated_layer
                            .custom_tile
                            .extend(layer.custom_tile.iter().cloned());

                        self.layer[index] = updated_layer;
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                self.layer.push(layer.clone());
            }
        }
    }
}
