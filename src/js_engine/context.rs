use crate::{
    js_engine::{engine::JsEngine, event::*, module::ModModule},
    unit::section::core::Core,
};
use bevy::prelude::*;
use boa_engine::{
    builtins::promise::PromiseState, js_string, module::SimpleModuleLoader, prelude::*,
    property::Attribute,
};
use boa_runtime::Console;
use std::{path::Path, rc::Rc};
use tokio::sync::mpsc::UnboundedSender as Sender;

pub(super) fn load_mod_libs(
    context: &mut Context,
    loader: Rc<SimpleModuleLoader>,
    simple_warfare_engine_js: &str,
) -> Result<Module, Box<dyn std::error::Error>> {
    let source = Source::from_reader(
        simple_warfare_engine_js.as_bytes(),
        Some(Path::new("./simple_warfare_engine.mjs")),
    );

    let module = Module::parse(source, None, context).unwrap();

    loader.insert(
        Path::new("./assets/mod_libs")
            .canonicalize()?
            .join("simple_warfare_engine.mjs"),
        module.clone(),
    );

    let promise_result = module
        .load(context)
        .then(
            Some(
                NativeFunction::from_copy_closure_with_captures(
                    |_, _, module, context| {
                        module.link(context).unwrap();
                        Ok(JsValue::undefined())
                    },
                    module.clone(),
                )
                .to_js_function(context.realm()),
            ),
            None,
            context,
        )
        .then(
            Some(
                NativeFunction::from_copy_closure_with_captures(
                    |_, _, module, context| Ok(module.evaluate(context).into()),
                    module.clone(),
                )
                .to_js_function(context.realm()),
            ),
            None,
            context,
        );

    context.run_jobs();

    match promise_result.state() {
        PromiseState::Pending => {
            return Err("module didn't execute!".into());
        }
        PromiseState::Fulfilled(v) => {
            assert_eq!(v, JsValue::undefined());
        }
        PromiseState::Rejected(err) => {
            return Err(JsError::from_opaque(err)
                .try_native(context)
                .unwrap()
                .into());
        }
    }
    Ok(module)
}

pub(super) fn add_runtime(context: &mut Context) {
    let console = Console::init(context);
    context
        .register_global_property(Console::NAME, console, Attribute::all())
        .expect("the console builtin shouldn't exist");
}

pub(super) fn register_class(context: &mut Context) {
    context
        .register_global_class::<Core>()
        .expect("the Core builtin shouldn't exist");
}

pub(super) fn process_js_event(
    engine: &mut JsEngine,
    event: JsEngineEvent,
    sender: &Sender<JsEngineEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let context = &mut engine.context;
    let module_map = &mut engine.module_map;
    let unit_map = &mut engine.unit_map;

    let libs_module = match module_map.get("simple_warfare_engine") {
        Some(mod_module) => mod_module[0].module.clone(),
        None => {
            return Err("libs didn't found".into()); // 或者执行其他恢复逻辑
        }
    };

    match event {
        JsEngineEvent::EngineInited => todo!(),
        JsEngineEvent::BuilderEvent(builder_event) => todo!(),
        JsEngineEvent::ModEvent(mod_event) => match mod_event {
            ModEvent::LoadMod(mod_enable, mod_info) => {
                for (js_asset, classes) in mod_enable.enable {
                    let module = Module::parse(
                        Source::from_reader(
                            js_asset.context.as_bytes(),
                            Some(Path::new(&js_asset.file_name)),
                        ),
                        Some(libs_module.realm().clone()),
                        context,
                    )
                    .unwrap();

                    let promise = module.load_link_evaluate(context);

                    context.run_jobs();

                    assert_eq!(
                        promise.state(),
                        PromiseState::Fulfilled(JsValue::undefined())
                    );

                    if let Some(modules) = module_map.get_mut(&mod_info.name) {
                        modules.push(ModModule::new(module.clone(), classes));
                    } else {
                        module_map.insert(
                            mod_info.name.clone(),
                            vec![ModModule::new(module.clone(), classes)],
                        );
                    }
                }
            }
            ModEvent::SpawnUnit(unit_class) => {
                let unit_from: Vec<&str> = unit_class.split(':').collect();
                if let Some(modules) = module_map.get(unit_from[0]) {
                    for module in modules {
                        if module.classes.contains(&unit_from[1].to_string()) {
                            let class = module
                                .module
                                .namespace(context)
                                .get(js_string!(unit_from[1]), context)
                                .unwrap();
                            let class_obj = class.as_object().ok_or("not found obj").unwrap();
                            let index = unit_map.len();
                            unit_map.insert(
                                index as u64,
                                class_obj
                                    .construct(&[], None, context)
                                    .expect("construct error"),
                            );
                        }
                    }
                }
            }
        },
    }
    Ok(())
}
