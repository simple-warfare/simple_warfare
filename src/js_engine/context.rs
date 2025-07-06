use crate::{
    custom_unit::{
        light2d::point_light2d::JsPointLight2d,
        physics::collider::JsCollider,
        section::{
            collider::JsColliders,
            core::Core,
            graphic::{Graphic, Graphics},
            light2d::JsPointLights2d,
            movement::Movement,
        },
        unit::SpawnedUnitData,
    },
    js_engine::{
        engine::JsEngine,
        event::*,
        global::class::entity::JsEntity,
        module::ModModule,
        signal::{EmitSignal, HostDefinedSignalSystem},
    },
};
use bevy::prelude::*;
use boa_engine::{
    JsResult,
    builtins::promise::PromiseState,
    js_string,
    object::{
        FunctionObjectBuilder,
        builtins::{JsArray, JsFunction, JsProxy},
    },
    prelude::*,
    value::{TryFromJs, TryIntoJs},
};
use rustc_hash::FxHashMap;
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
    let unit_map = &mut engine.unit_map;
    let entity_map = &mut engine.entity_map;

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
                        let movement = Movement::try_from_js(
                            &unit_proxy.get(js_string!("movement"), context)?,
                            context,
                        )?;

                        let colliders = JsColliders::new(Vec::<JsCollider>::try_from_js(
                            &JsValue::Object(
                                unit_proxy
                                    .get(js_string!("colliders"), context)?
                                    .to_object(context)?,
                            ),
                            context,
                        )?);

                        let point_lights =
                            JsPointLights2d::new(Vec::<JsPointLight2d>::try_from_js(
                                &JsValue::Object(
                                    unit_proxy
                                        .get(js_string!("pointLights"), context)?
                                        .to_object(context)?,
                                ),
                                context,
                            )?);

                        let created_signal = unit_proxy
                            .get(js_string!("created"), context)?
                            .to_object(context)?
                            .clone();

                        context
                            .realm()
                            .host_defined_mut()
                            .get_mut::<HostDefinedSignalSystem>()
                            .unwrap()
                            .insert_emit_signal(EmitSignal::new(created_signal, &[]));
                        request_sender
                            .send(JsEngineRequestEvent::SignalEmit)
                            .unwrap();

                        unit_map.insert(entity, unit_proxy);
                        entity_map.insert(js_entity, entity);

                        response_sender
                            .send(JsEngineResponseEvent::SpawnedUnit(
                                entity,
                                module_path,
                                SpawnedUnitData::new(
                                    core,
                                    graphics,
                                    movement,
                                    colliders,
                                    point_lights,
                                ),
                            ))
                            .unwrap();
                    }
                }
            }
        }
        JsEngineRequestEvent::GetEntityToTeleport(js_entity, vec2) => {
            response_sender
                .send(JsEngineResponseEvent::GetedEntityToTeleport(
                    js_entity,
                    *entity_map.get(&js_entity).unwrap(),
                    vec2,
                ))
                .unwrap();
        }
        JsEngineRequestEvent::SignalEmit => {
            if context
                .realm()
                .host_defined()
                .get::<HostDefinedSignalSystem>()
                .unwrap()
                .signal_emit_queue
                .is_empty()
            {
                return Ok(());
            }
            let signal_emit_queue: Vec<EmitSignal> = context
                .realm()
                .host_defined_mut()
                .get_mut::<HostDefinedSignalSystem>()
                .unwrap()
                .signal_emit_queue
                .drain(..)
                .collect();

            for emit_signal in signal_emit_queue {
                let signal = &emit_signal.signal;

                let connect_array = JsArray::from_object(
                    signal
                        .get(js_string!("connectArray"), context)?
                        .to_object(context)?,
                )?;

                let func = connect_array
                    .get(js_string!("0"), context)?
                    .as_function()
                    .unwrap();
                func.call(&JsValue::Undefined, &emit_signal.args, context)?;
            }
        }
        JsEngineRequestEvent::SignalConnect => todo!(),
    }
    Ok(())
}
