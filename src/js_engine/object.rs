use bevy::platform::collections::HashMap;
use boa_engine::prelude::*;

pub struct ModObject {
    pub id: u64,
    pub object: JsObject,
}

pub const OBJECT_MAP: HashMap<u64, ModObject> = HashMap::new();
