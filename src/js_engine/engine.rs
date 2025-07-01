use std::rc::Rc;

use bevy::{ecs::entity::Entity, platform::collections::HashMap};
use boa_engine::{module::SimpleModuleLoader, object::builtins::JsProxy, prelude::*};

use crate::js_engine::{
    add_runtime,
    loader::{SimpleWarfareModuleLoader, SwModuleJobQueue},
    module::ModModule,
    register_class,
};

pub struct JsEngine {
    pub(super) context: Context,
    pub(super) module_map: HashMap<String, Vec<ModModule>>,
    pub(super) unit_map: HashMap<Entity, JsProxy>,
}

impl JsEngine {
    pub fn new(loader: SimpleWarfareModuleLoader) -> Self {
        let context_builder = Context::builder().job_queue(Rc::new(SwModuleJobQueue::new()));
        let loader = Rc::new(loader);

        let mut context = context_builder
            .module_loader(loader.clone())
            .build()
            .expect("Build Js Context error!");
        add_runtime(&mut context);
        register_class(&mut context);

        Self {
            context,
            module_map: HashMap::new(),
            unit_map: HashMap::new(),
        }
    }
}
