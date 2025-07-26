use bevy::{ecs::entity::Entity, log::info, reflect::Reflect};
use boa_engine::{
    JsData,
    value::{TryFromJs, TryIntoJs},
};
use serde::{Deserialize, Serialize};

/// Js端的Entity,但不可以直接转换成Entity
/// TODO:在Js端使用Json存储Entity得了
#[derive(
    Debug, Clone, TryFromJs, JsData, TryIntoJs, Eq, Hash, PartialEq, Reflect, Deserialize, Serialize,
)]
pub struct JsEntity {
    inner: String,
    pub index: u32,
    pub generation: u32,
}

impl JsEntity {
    pub fn from_entity(entity: &Entity) -> Self {
        Self {
            inner: serde_json::to_string(&entity).unwrap(),
            index: entity.index(),
            generation: entity.generation(),
        }
    }
    pub fn to_entity(&self) -> Entity {
        serde_json::from_str::<Entity>(&self.inner).unwrap()
    }
}
