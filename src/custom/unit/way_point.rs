use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Debug, Clone, Reflect, Copy)]
pub enum WayPoint {
    Move(Vec2),
    Attack()
}

#[derive(Debug, Default, Component, Reflect, Clone)]
pub struct WayPointQueue {
    pub data: VecDeque<WayPoint>,
}
