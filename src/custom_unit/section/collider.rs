use crate::{custom_unit::physics::collider::Collider, statistics::Avian2dCollider};
use bevy::prelude::*;
use boa_engine::value::TryFromJs;
#[derive(Debug, Default, Clone, Component, Reflect, TryFromJs)]
pub struct Colliders {
    pub data: Vec<Collider>,
}

impl Colliders {
    pub fn new(colliders: Vec<Collider>) -> Self {
        Self { data: colliders }
    }

    pub fn to_avian2d(&self) -> Vec<Avian2dCollider> {
        self.data
            .iter()
            .map(|collider| collider.to_avian2d())
            .collect()
    }
}
