use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum WayPointType {
    Move(Vec2),
}

#[derive(Debug, Component, Clone, Copy)]
pub struct ActiveWayPoint(pub WayPointType);
