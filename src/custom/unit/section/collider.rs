use crate::custom::unit::physics::collider::JsCollider;
use avian2d::prelude::Collider;
use bevy::prelude::*;
use boa_engine::value::TryFromJs;
use serde::{Deserialize, Serialize};

/// mod单位拥有一个Array来存储碰撞体,在rust中我们将使用Vec
#[derive(Debug, Default, Clone, Serialize, Deserialize, Component, Reflect, TryFromJs)]
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
