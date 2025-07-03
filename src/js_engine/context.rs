use crate::{
    js_engine::{engine::JsEngine, event::*, module::ModModule},
    unit::{
        custom_unit::SpawnedUnitData,
        section::{
            core::Core,
            graphic::{Graphic, Graphics},
        },
    },
};
use bevy::prelude::*;
use boa_engine::{
    JsResult,
    builtins::promise::PromiseState,
    js_string,
    object::builtins::{JsArray, JsProxy},
    prelude::*,
    value::TryFromJs,
};
use std::{
    path::Path,
    sync::{Arc, mpsc::Sender},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsEngineError {
    #[error("JsError: {0}")]
    Js(#[from] boa_engine::error::JsError),
    #[error("Std Error: {0}")]
    Std(#[from] Box<dyn std::error::Error>),
}

pub(super) fn process_js_event(
    engine: &mut JsEngine,
    event: JsEngineRequestEvent,
    sender: Arc<Sender<JsEngineResponseEvent>>,
) -> JsResult<()> {
    let context = &mut engine.context;
    let module_map = &mut engine.module_map;
    let unit_map = &mut engine.unit_map;

    match event {
        JsEngineRequestEvent::LoadMod(mod_enable, mod_info) => {
            for (js_asset, classes) in mod_enable.enable {
                let module = Module::parse(
                    Source::from_reader(
                        js_asset.context.as_bytes(),
                        Some(&Path::new(&js_asset.from).join(js_asset.file_name)),
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
        JsEngineRequestEvent::SpawnUnit(entity, unit_str) => {
            info!("SpawnUnit:{}", unit_str);
            let unit_from: Vec<&str> = unit_str.split(':').collect();
            if let Some(modules) = module_map.get(unit_from[0]) {
                for module in modules {
                    let target_class = unit_from[1].to_string();
                    if module.classes.contains(&target_class) {
                        let module_path =
                            module.module.path().unwrap().to_string_lossy().into_owned();
                        let class = module
                            .module
                            .namespace(context)
                            .get(js_string!(unit_from[1]), context)?;

                        let class_obj = class
                            .to_object(context)?
                            .construct(&[], None, context)
                            .expect("construct error");

                        let unit_proxy = JsProxy::from_object(
                            class_obj
                                .get(js_string!("get_proxy"), context)?
                                .as_callable()
                                .ok_or(JsError::from_opaque(
                                    js_string!(format!("the vaule is not callable",)).into(),
                                ))?
                                .call(&JsValue::Object(class_obj), &[], context)?
                                .to_object(context)?
                                .clone(),
                        )?;

                        let graphics = Graphics::new(Vec::<Graphic>::try_from_js(
                            &JsValue::Object(
                                unit_proxy
                                    .get(js_string!("graphics"), context)?
                                    .to_object(context)?,
                            ),
                            context,
                        )?);

                        let core = Core::try_from_js(
                            &unit_proxy.get(js_string!("core"), context)?,
                            context,
                        )?;

                        unit_map.insert(entity, unit_proxy);
                        sender
                            .send(JsEngineResponseEvent::SpawnedUnit(
                                entity,
                                module_path,
                                SpawnedUnitData::new(core, graphics),
                            ))
                            .unwrap();
                    }
                }
            }
        }
    }
    Ok(())
}
