use std::rc::Rc;

use bevy::platform::collections::HashMap;
use boa_engine::{module::SimpleModuleLoader, prelude::*};

use crate::js_engine::{add_runtime, load_mod_libs, module::ModModule, register_class};

pub struct JsEngine {
    pub(super) context: Context,
    pub(super) module_map: HashMap<String, ModModule>,
}

impl JsEngine {
    pub fn new(default_module_code: &str) -> Self {
        let mut module_map = HashMap::new();

        let context_builder = Context::builder();
        let loader =
            Rc::new(SimpleModuleLoader::new("./assets/mod_libs").expect("load mod_libs error"));

        let mut context = context_builder
            .module_loader(loader.clone())
            .build()
            .expect("Build Js Context error!");
        add_runtime(&mut context);
        register_class(&mut context);

        let module = load_mod_libs(&mut context, loader.clone(), default_module_code)
            .expect("load module error");

        module_map.insert(
            "simple_warfare_engine".to_string(),
            ModModule::new(module, vec!["Core".to_string(), "CustomUnit".to_string()]),
        );
        Self {
            context,
            module_map,
        }
    }
}
