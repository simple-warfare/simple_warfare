use std::{cell::RefCell, rc::Rc};

use bevy::{platform::collections::HashMap, prelude::Entity};
use boa_engine::{object::builtins::JsProxy, prelude::*};
use rustc_hash::FxHashMap;

use crate::js_engine::global::class::entity::JsEntity;

#[derive(Default, Trace, Finalize, JsData)]
pub struct UnitMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<HashMap<JsEntity, JsProxy>>>,
}
#[derive(Default, Trace, Finalize, JsData)]
pub struct EntityMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<HashMap<JsEntity, Entity>>>,
}
#[derive(Default, Trace, Finalize, JsData)]
pub struct SelectedSignalMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<FxHashMap<JsEntity, JsObject>>>,
}
#[derive(Default, Trace, Finalize, JsData)]
pub struct OnUnitEnterSignalMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<FxHashMap<JsEntity, JsObject>>>,
}
#[derive(Default, Trace, Finalize, JsData)]
pub struct OnUnitExitSignalMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<FxHashMap<JsEntity, JsObject>>>,
}
#[derive(Default, Trace, Finalize, JsData)]
pub struct SignalEntityMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<FxHashMap<JsEntity, JsObject>>>,
}
