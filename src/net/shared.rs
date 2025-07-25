use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};
use std::net::Ipv6Addr;

pub const SERVER_HOST: Ipv6Addr = Ipv6Addr::LOCALHOST;
pub const LOCAL_BIND_IP: Ipv6Addr = Ipv6Addr::UNSPECIFIED;
pub const SERVER_PORT: u16 = 6000;

// Defines the amount of time that should elapse between each physics step.
pub const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Resource, Default)]
pub struct UnitMapping {
   pub map: HashMap<UnitId, Vec<Entity>>,
}

pub type UnitId = Entity;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    pub name: String,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "Player".to_string(),
        }
    }
}

impl UnitMapping {
    pub fn new_unit(&mut self, commands: &mut Commands) -> UnitId {
        let unit_id = commands.spawn_empty().id();
        self.map.insert(unit_id, vec![]);
        unit_id
    }

    pub fn add_entity(&mut self, unit_id: Entity, entity: Entity) {
        self.map.entry(unit_id).or_default().push(entity);
    }

    pub fn get_units(&self, unit_id: UnitId) -> Option<&Vec<Entity>> {
        self.map.get(&unit_id)
    }

    pub fn remove_unit(&mut self, unit_id: UnitId, entity: Entity) {
        if let Some(entities) = self.map.get_mut(&unit_id) {
            entities.retain(|&e| e != entity);
            if entities.is_empty() {
                self.map.remove(&unit_id);
            }
        }
    }
}
