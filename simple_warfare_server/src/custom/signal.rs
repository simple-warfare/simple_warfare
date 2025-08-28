use std::time::Duration;

use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
    time::common_conditions::on_real_timer,
};
use boa_engine::{
    JsResult, js_string,
    object::builtins::JsMap,
    prelude::*,
    value::TryFromJs,
};
use simple_warfare_server_macros::TryFromAndIntoJs;

use crate::{
    bevy_ext::prelude::*,
    js_engine::{JsEngineRequestSender, event::JsEngineRequestEvent, signal::JsSignalType},
};

use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
pub struct JsSignalStorage {
    pub default_signal_map: HashMap<JsSignalType, JsSignal>,
    pub custom_signal_set: HashSet<JsSignal>,
}

#[derive(Debug, Hash, Clone, Reflect, Serialize, Deserialize, TryFromAndIntoJs, PartialEq, Eq)]
pub struct JsSignal {
    #[boa(
        from_js_with = "entity_try_from_js",
        into_js_with = "entity_try_into_js"
    )]
    pub entity: Entity,
}

impl TryFromJs for JsSignalStorage {
    fn try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Self> {
        let signal_storage = value.to_object(context)?;
        let signal_map = JsMap::from_object(
            signal_storage
                .get(js_string!("signalMap"), context)?
                .to_object(context)?,
        )?;
        let mut default_signal_map = HashMap::new();
        let mut custom_signal_set = HashSet::new();

        signal_map.for_each_native(|key, value| {
            let signal = JsSignal::try_from_js(&value, context)?;
            let signal_type = JsSignalType::try_from_js(&key, context)?;
            if JsSignalType::Custom == signal_type {
                custom_signal_set.insert(signal);
            } else {
                default_signal_map.insert(signal_type, signal);
            }
            JsResult::Ok(())
        })?;

        JsResult::Ok(Self {
            default_signal_map,
            custom_signal_set,
        })
    }
}

pub struct JsSignalPlugin;

impl Plugin for JsSignalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            emit_fixed_update_signal.run_if(
                resource_exists::<JsEngineRequestSender>
                    .and(on_real_timer(Duration::from_secs_f32(0.1))),
            ),
        );
    }
}

fn emit_fixed_update_signal(
    time: Res<Time>,
    js_engine_request_sender: Res<JsEngineRequestSender>,
    js_signal_storage_query: Query<&JsSignalStorage>,
) -> Result {
    let delta_time = time.delta_secs();
    for js_signal_storage in js_signal_storage_query {
        if let Some(fixed_update_signal) = js_signal_storage
            .default_signal_map
            .get(&JsSignalType::FixedUpdate)
        {
            js_engine_request_sender
                .0
                .send(JsEngineRequestEvent::fixed_update_signal(
                    fixed_update_signal.entity,
                    delta_time,
                ))?;
        }
    }

    Ok(())
}
