use bevy::prelude::*;

#[derive(Event, Clone)]
pub enum SmilodonEngineEvent {
    EngineInited,
    BuilderEvent(BuilderEvent),
    LoadEvent()
}

#[derive(Clone)]
pub enum BuilderEvent {
}

