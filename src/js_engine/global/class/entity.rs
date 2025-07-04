use bevy::ecs::entity::Entity;
use boa_engine::value::{TryFromJs, TryIntoJs};

#[derive(Debug, Clone, Copy, TryFromJs, TryIntoJs, Eq, Hash, PartialEq)]
pub struct JsEntity {
    pub index: u32,
    pub generation: u32,
}

impl JsEntity {
    pub fn from_entity(entity: &Entity) -> Self {
        Self {
            index: entity.index(),
            generation: entity.generation(),
        }
    }
}
