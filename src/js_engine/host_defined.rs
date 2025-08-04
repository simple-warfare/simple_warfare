use std::{cell::RefCell, rc::Rc, sync::Arc};

use bevy::{platform::collections::HashMap, prelude::Entity};
use boa_engine::{object::builtins::JsProxy, prelude::*};
use rustc_hash::FxHashMap;

use crate::custom::{CustomTypedId, unit::CustomInnerInfo};

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

#[derive(Default, Trace, Finalize, JsData)]
pub struct CustomInnerInfoMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<FxHashMap<CustomTypedId, (Vec<Entity>, Arc<CustomInnerInfo>)>>>,
}

#[derive(Default, Trace, Finalize, JsData)]
pub struct ModulePathToCustomTypedIdMap {
    #[unsafe_ignore_trace]
    pub map: Rc<RefCell<FxHashMap<String, CustomTypedId>>>,
}
