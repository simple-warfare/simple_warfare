use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};
use boa_engine::{
    JsResult, js_string,
    object::{FunctionObjectBuilder, builtins::JsMap},
    prelude::*,
    value::TryFromJs,
};
use simple_warfare_macros::TryFromAndIntoJs;

use crate::{bevy_ext::prelude::*, js_engine::signal::JsDefaultSignalType};

use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
pub struct SignalStorage {
    pub default_signal_map: HashMap<JsDefaultSignalType, Entity>,
    pub custom_signal_set: HashSet<Entity>,
}


pub struct Signal{
    
}

impl TryFromJs for SignalStorage {
    fn try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Self> {
        let signal_storage = value.to_object(context)?;
        let signal_map = JsMap::from_object(
            signal_storage
                .get(js_string!("signalMap"), context)?
                .to_object(context)?,
        )?;
        let mut default_signal_map = HashMap::new();
        let mut custom_signal_set = HashSet::new();

        let callback = FunctionObjectBuilder::new(
            context.realm(),
            NativeFunction::from_fn_ptr(|_this, args, context| {
                info!("{:?}", args);
                JsResult::Ok(JsValue::Undefined)
            }),
        )
        .build();

        signal_map.for_each(callback, JsValue::Undefined, context)?;

        JsResult::Ok(Self {
            default_signal_map,
            custom_signal_set,
        })
    }
}
