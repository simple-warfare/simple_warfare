use std::{cell::RefCell, rc::Rc};

use bevy::{platform::collections::HashMap, prelude::Entity};
use boa_engine::{object::builtins::JsProxy, prelude::*};
use rustc_hash::FxHashMap;

#[derive(Default, Trace, Finalize, JsData)]
pub struct UnitMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<HashMap<Entity, JsProxy>>>,
}

#[derive(Default, Trace, Finalize, JsData)]
pub struct JsObjectMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<HashMap<Entity, JsObject>>>,
}
#[derive(Default, Trace, Finalize, JsData)]
pub struct JsProxyMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<HashMap<Entity, JsProxy>>>,
}
#[derive(Default, Trace, Finalize, JsData)]
pub struct SelectedSignalMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<FxHashMap<Entity, JsObject>>>,
}
#[derive(Default, Trace, Finalize, JsData)]
pub struct OnUnitEnterSignalMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<FxHashMap<Entity, JsObject>>>,
}
#[derive(Default, Trace, Finalize, JsData)]
pub struct OnUnitExitSignalMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<FxHashMap<Entity, JsObject>>>,
}
#[derive(Default, Trace, Finalize, JsData)]
pub struct SignalEntityMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<FxHashMap<Entity, JsObject>>>,
}
