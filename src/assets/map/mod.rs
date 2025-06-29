pub mod ldtk;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MapKind {
    Ldtk,
    Tiled,
}

pub trait Map {
    
}
