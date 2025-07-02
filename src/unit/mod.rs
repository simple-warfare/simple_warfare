pub mod custom_unit;
pub mod section;
use bevy::prelude::*;

use crate::js_engine::event::JsEngineResponseEvent;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_new_unit.run_if(on_event::<JsEngineResponseEvent>),
        );
    }
}

fn check_new_unit(mut reader: EventReader<JsEngineResponseEvent>) {
    for event in reader.read(){
        if let JsEngineResponseEvent::SpawnedUnit(entity) = *event{
            info!("new Unit Spawned")
        }
    }
}
