use crate::custom_unit::physics::collider::JsCollider;
use avian2d::prelude::Collider;
use bevy::prelude::*;
use boa_engine::value::TryFromJs;
#[derive(Debug, Default, Clone, Component, Reflect, TryFromJs)]
pub struct JsColliders {
    pub data: Vec<JsCollider>,
}

impl JsColliders {
    pub fn new(colliders: Vec<JsCollider>) -> Self {
        Self { data: colliders }
    }

    pub fn to_avian2d(&self) -> Vec<Collider> {
        self.data
            .iter()
            .map(|collider| collider.to_avian2d())
            .collect()
    }
}
