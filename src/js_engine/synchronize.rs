use bevy::prelude::*;
use boa_engine::{Context, JsNativeError, JsResult, JsValue, value::TryFromJs};

use crate::{
    custom::unit::section::{core::Core, movement::Movement},
    js_engine::{
        JsEngineRequestSender,
        event::{JsEngineRequestEvent, JsEngineResponseEvent},
    },
};

pub enum SynchronizeDataFromJs {
    //Section
    Core(Core),
    Movement(Movement),
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub enum SynchronizeDataFromJsType {
    Core,
    Movement,
}

impl TryFromJs for SynchronizeDataFromJsType {
    fn try_from_js(value: &JsValue, _context: &mut Context) -> JsResult<Self> {
        if let JsValue::String(synchronize_type) = value {
            match synchronize_type.to_std_string_lossy().as_str() {
                "Core" => Ok(SynchronizeDataFromJsType::Core),
                "Movement" => Ok(SynchronizeDataFromJsType::Movement),
                _ => Err(JsNativeError::typ()
                    .with_message("cannot convert value to a SynchronizeDataFromJsType")
                    .into()),
            }
        } else {
            Err(JsNativeError::typ()
                .with_message(
                    "cannot convert a value which is not a string to a SynchronizeDataFromJsType",
                )
                .into())
        }
    }
}
pub struct SynchronizePlugin;

impl Plugin for SynchronizePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            synchronize_data.run_if(resource_exists::<JsEngineRequestSender>),
        )
        .add_systems(Update, handle_synchronize_core_event);
    }
}

pub fn synchronize_data(
    js_engine_requests_sender: Res<JsEngineRequestSender>,
    synchronize_datas: Query<&Core, Changed<Core>>,
) -> Result {
    for core in synchronize_datas {
        js_engine_requests_sender
            .0
            .send(JsEngineRequestEvent::synchronize_from_js(
                SynchronizeDataFromJs::Core(core.clone()),
            ))?;
    }
    Ok(())
}

fn handle_synchronize_core_event(
    mut synchronize_core_events: EventReader<JsEngineResponseEvent>,
    mut cores: Query<&mut Core>,
) -> Result {
    for event in synchronize_core_events.read() {
        if let JsEngineResponseEvent::SynchronizeCoreFromJs { data } = event {
            let mut target_core = cores.get_mut(data.entity)?;
            *target_core = data.clone();
        }
    }
    Ok(())
}


fn handle_synchronize_movement_event(
    mut synchronize_core_events: EventReader<JsEngineResponseEvent>,
    mut cores: Query<&mut Core>,
) -> Result {
    for event in synchronize_core_events.read() {
        if let JsEngineResponseEvent::SynchronizeCoreFromJs { data } = event {
            let mut target_core = cores.get_mut(data.entity)?;
            *target_core = data.clone();
        }
    }
    Ok(())
}