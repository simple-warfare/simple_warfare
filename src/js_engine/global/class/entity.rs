use bevy::{ecs::entity::Entity, reflect::Reflect};
use boa_engine::{
    JsData,
    value::{TryFromJs, TryIntoJs},
};
use serde::{Deserialize, Serialize};
#[derive(
    Debug,
    Clone,
    Copy,
    TryFromJs,
    JsData,
    TryIntoJs,
    Eq,
    Hash,
    PartialEq,
    Reflect,
    Deserialize,
    Serialize,
)]
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
