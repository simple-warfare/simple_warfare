use crate::{
    custom_unit::{section::Section, unit::SpawnedUnitData},
    js_engine::{engine::JsEngine, event::*, global::class::entity::JsEntity, module::ModModule},
};
use bevy::prelude::*;
use boa_engine::{
    JsResult,
    builtins::promise::PromiseState,
    js_string,
    object::builtins::{JsArray, JsProxy},
    prelude::*,
    value::TryIntoJs,
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
    request_sender: Arc<Sender<JsEngineRequestEvent>>,
    response_sender: Arc<Sender<JsEngineResponseEvent>>,
) -> JsResult<()> {
    let context = &mut engine.context;
    let module_map = &mut engine.module_map;
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
            let unit_map = &mut engine.unit_map;
            let entity_map = &mut engine.entity_map;
            let selected_signal_map = &mut engine.selected_signal_map;

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

                        let js_entity = JsEntity::from_entity(&entity);
                        let class_obj = class
                            .to_object(context)?
                            .construct(&[js_entity.try_into_js(context)?], None, context)
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

                        let section = Section::try_from_proxy(&unit_proxy, context)?;

                        let selected_signal = unit_proxy
                            .get(js_string!("selected"), context)?
                            .to_object(context)?
                            .clone();

                        selected_signal_map.insert(js_entity.clone(), selected_signal);

                        let created_signal = unit_proxy
                            .get(js_string!("created"), context)?
                            .to_object(context)?
                            .clone();

                        emit_signal(&created_signal, &[], context)?;

                        unit_map.insert(entity, unit_proxy);
                        entity_map.insert(js_entity, entity);

                        response_sender
                            .send(JsEngineResponseEvent::SpawnedUnit(
                                entity,
                                module_path,
                                SpawnedUnitData::new(section),
                            ))
                            .unwrap();
                    }
                }
            }
        }
        JsEngineRequestEvent::GetEntityToTeleport(js_entity, vec2) => {
            let entity_map = &engine.entity_map;
            response_sender
                .send(JsEngineResponseEvent::GetedEntityToTeleport(
                    js_entity,
                    *entity_map.get(&js_entity).unwrap(),
                    vec2,
                ))
                .unwrap();
        }
        JsEngineRequestEvent::SelectedSignalEmit => {
            let selected_signal_map = &engine.selected_signal_map;
            selected_signal_map.iter().for_each(|(_, signal)| {
                let connect_array = JsArray::from_object(
                    signal
                        .get(js_string!("connectArray"), context)
                        .unwrap()
                        .to_object(context)
                        .unwrap(),
                )
                .unwrap();

                let func = connect_array
                    .get(js_string!("0"), context)
                    .unwrap()
                    .as_function()
                    .unwrap();
                func.call(&JsValue::Undefined, &[], context).unwrap();
            });
        }
    }
    Ok(())
}

fn emit_signal(signal: &JsObject, args: &[JsValue], context: &mut Context) -> JsResult<()> {
    let connect_array = JsArray::from_object(
        signal
            .get(js_string!("connectArray"), context)?
            .to_object(context)?,
    )?;

    for i in 0..connect_array.length(context)? {
        let func = connect_array.at(i as i64, context)?.as_function().unwrap();
        func.call(&JsValue::Undefined, args, context)?;
    }

    Ok(())
}
