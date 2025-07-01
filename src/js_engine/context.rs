use crate::{
    js_engine::{engine::JsEngine, event::*, module::ModModule},
    unit::section::core::Core,
};
use bevy::prelude::*;
use boa_engine::{
    builtins::promise::PromiseState, js_string, object::builtins::JsProxy, prelude::*,
    property::Attribute,
};
use boa_runtime::Console;
use std::{
    path::Path,
    sync::{Arc, mpsc::Sender},
};

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
    event: JsEngineRequestEvent,
    sender: Arc<Sender<JsEngineResponeEvent>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let context = &mut engine.context;
    let module_map = &mut engine.module_map;
    let unit_map = &mut engine.unit_map;

    match event {
        JsEngineRequestEvent::ModEvent(mod_event) => match mod_event {
            ModEvent::LoadMod(mod_enable, mod_info) => {
                for (js_asset, classes) in mod_enable.enable {
                    let module = Module::parse(
                        Source::from_reader(
                            js_asset.context.as_bytes(),
                            Some(Path::new(&js_asset.file_name)),
                        ),
                        None,
                        context,
                    )?;

                    let promise = module.load_link_evaluate(context);

                    context.run_jobs();

                    match promise.state() {
                        PromiseState::Pending => panic!("module didn't execute!"),
                        PromiseState::Fulfilled(v) => {
                            assert_eq!(v, JsValue::undefined())
                        }
                        PromiseState::Rejected(err) => {
                            panic!("{}", err.display());
                        }
                    }

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
            ModEvent::SpawnUnit(entity, unit_str) => {
                info!("SpawnUnit:{}", unit_str);
                let unit_from: Vec<&str> = unit_str.split(':').collect();
                if let Some(modules) = module_map.get(unit_from[0]) {
                    info!("found the module:{}", unit_from[0]);
                    for module in modules {
                        if module.classes.contains(&unit_from[1].to_string()) {
                            let class = module
                                .module
                                .namespace(context)
                                .get(js_string!(unit_from[1]), context)?;

                            let class_obj = class
                                .as_object()
                                .ok_or("not found obj")?
                                .construct(&[], None, context)
                                .expect("construct error");

                            info!("class_obj:{:?}", class_obj);

                            let unit_proxy = JsProxy::from_object(
                                class_obj
                                    .get(js_string!("get_proxy"), context)?
                                    .as_callable()
                                    .unwrap()
                                    .call(&JsValue::Object(class_obj), &[], context)
                                    .unwrap()
                                    .as_object()
                                    .unwrap()
                                    .clone(),
                            )
                            .unwrap();

                            info!("{:?}", unit_proxy.get(js_string!("name"), context).unwrap());

                            unit_map.insert(entity, unit_proxy);
                        }
                    }
                }
            }
        },
    }
    Ok(())
}
