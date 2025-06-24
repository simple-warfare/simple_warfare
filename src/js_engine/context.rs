use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use boa_engine::{
    builtins::promise::PromiseState, js_string, module::SimpleModuleLoader, prelude::*,
    property::Attribute,
};
use boa_runtime::Console;
use std::{path::Path, rc::Rc};

use crate::{js_engine::event::*, unit::section::core::Core};

pub(super) fn load_mod_libs(
    context: &mut Context,
    loader: Rc<SimpleModuleLoader>,
    simple_warfare_engine_js: String,
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
    context: &mut Context,
    module_map: &mut HashMap<&str, Module>,
    event: JsEngineEvent,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(libs_module) = module_map.get("simple_warfare_engine") {
        match event {
            JsEngineEvent::EngineInited => todo!(),
            JsEngineEvent::BuilderEvent(builder_event) => todo!(),
            JsEngineEvent::ModEvent(mod_event) => match mod_event {
                ModEvent::LoadJs(js_asset) => {
                    let module = Module::parse(
                        Source::from_reader(
                            js_asset.context.as_bytes(),
                            Some(Path::new("./tank.mjs")),
                        ),
                        Some(libs_module.realm().clone()),
                        context,
                    )
                    .unwrap();
                    let promise = module.load_link_evaluate(context);
                    //tx.send(SmilodonEngineEvent::EngineInited)?;
                    context.run_jobs();

                    assert_eq!(
                        promise.state(),
                        PromiseState::Fulfilled(JsValue::undefined())
                    );

                    let binding = module
                        .namespace(context)
                        .get(js_string!("Tank"), context)
                        .unwrap();
                    let tank_obj = binding.as_object().ok_or("not found obj").unwrap();

                    let tank = tank_obj.construct(&[], None, context).unwrap();
                    info!("{:?}", tank.get(js_string!("name"), context)?);

                    tank.set(js_string!("name"), js_string!("超级坦克"), true, context)?;
                    info!("{:?}", tank.get(js_string!("name"), context)?);
                }
                ModEvent::EnableUnit(_) => todo!(),
            },
        }
        Ok(())
    } else {
        Err("libs didn't found".into())
    }
}
