use bevy::prelude::*;
use boa_engine::{Context, JsNativeError, JsResult, JsValue, value::TryFromJs};

use crate::{
    custom::unit::{
        section::{core::Core, movement::Movement},
        transform::transform::JsTransform,
    },
    js_engine::{
        JsEngineRequestSender,
        event::{JsEngineRequestEvent, JsEngineResponseEvent},
    },
};

pub enum SynchronizeData {
    //Section
    Core(Core),
    Movement(Movement),
    //Transform
    Transform(JsTransform),
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub enum SynchronizeDataType {
    Core,
    Movement,
}

impl TryFromJs for SynchronizeDataType {
    fn try_from_js(value: &JsValue, _context: &mut Context) -> JsResult<Self> {
        if let JsValue::String(synchronize_type) = value {
            match synchronize_type.to_std_string_lossy().as_str() {
                "Core" => Ok(SynchronizeDataType::Core),
                "Movement" => Ok(SynchronizeDataType::Movement),
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
            synchronize_core.run_if(resource_exists::<JsEngineRequestSender>),
        )
        .add_systems(Update, handle_synchronize_core_event)
        .add_systems(
            PostUpdate,
            synchronize_transform
                .after(TransformSystem::TransformPropagate)
                .run_if(resource_exists::<JsEngineRequestSender>),
        );
    }
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

pub fn synchronize_core(
    js_engine_requests_sender: Res<JsEngineRequestSender>,
    changed_cores: Query<&Core, Changed<Core>>,
) -> Result {
    for core in changed_cores {
        js_engine_requests_sender
            .0
            .send(JsEngineRequestEvent::synchronize_to_js(
                SynchronizeData::Core(core.clone()),
            ))?;
    }
    Ok(())
}

fn synchronize_transform(
    js_engine_requests_sender: Res<JsEngineRequestSender>,
    changed_transforms: Query<(&Transform, &mut JsTransform), Changed<Transform>>,
) -> Result {
    for (transform, mut js_transform) in changed_transforms {
        js_transform.update(transform);
        js_engine_requests_sender
            .0
            .send(JsEngineRequestEvent::synchronize_to_js(
                SynchronizeData::Transform(js_transform.clone()),
            ))?;
    }
    Ok(())
}
