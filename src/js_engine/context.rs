use crate::{
    custom::{
        CustomModEnableJs,
        unit::{section::Section, unit::SpawnedUnitData},
    },
    js_engine::{
        engine::JsEngine,
        event::*,
        global::class::entity::JsEntity,
        host_defined::*,
        module::ModModule,
        sw::{LookType, TeleportType},
        synchronize::SynchronizeData,
    },
};
use bevy::prelude::*;
use boa_engine::{
    JsResult,
    builtins::promise::PromiseState,
    js_string,
    object::builtins::{JsArray, JsProxy},
    prelude::*,
    value::{TryFromJs, TryIntoJs},
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
    _request_sender: Arc<Sender<JsEngineRequestEvent>>,
    response_sender: Arc<Sender<JsEngineResponseEvent>>,
) -> JsResult<()> {
    let context = &mut engine.context;
    let module_map = &mut engine.module_map;
    let custom_typed_id_generator = &mut engine.custom_typed_id_generator;

    match event {
        JsEngineRequestEvent::LoadMod(custom_mod_asset) => {
            let custom_typed_id = *custom_typed_id_generator;
            *custom_typed_id_generator += 1;

            let mod_info = custom_mod_asset.info.clone();
            for CustomModEnableJs {
                js_asset,
                enable_class,
            } in custom_mod_asset.custom_mod_enable_js
            {
                let module = Module::parse(
                    Source::from_reader(
                        js_asset.context.as_bytes(),
                        Some(Path::new(&js_asset.path)),
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
                // 将模块添加到模块映射中
                module_map
                    .entry_ref(&mod_info.name)
                    .or_default()
                    .push(ModModule::new(module.clone(), enable_class));
                let module_path = module.path().unwrap().to_string_lossy().into_owned();
                context
                    .realm()
                    .host_defined_mut()
                    .get_mut::<ModulePathToCustomTypedIdMap>()
                    .unwrap()
                    .map
                    .borrow_mut()
                    .insert(module_path, custom_typed_id);
            }
        }
        JsEngineRequestEvent::SpawnUnit { unit_id, unit_str } => {
            let unit_from: Vec<&str> = unit_str.split(':').collect();
            if let Some(modules) = module_map.get(unit_from[0]) {
                for mod_module in modules {
                    let target_class = unit_from[1].to_string();
                    if mod_module.classes.contains(&target_class) {
                        let module_path = mod_module
                            .module
                            .path()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned();

                        let custom_typed_id = *context
                            .realm()
                            .host_defined()
                            .get::<ModulePathToCustomTypedIdMap>()
                            .unwrap()
                            .map
                            .borrow()
                            .get(&module_path)
                            .unwrap();

                        let class = mod_module
                            .module
                            .namespace(context)
                            .get(js_string!(unit_from[1]), context)?;

                        let module_parent_path = Path::new(&module_path)
                            .parent()
                            .unwrap()
                            .to_string_lossy()
                            .to_string();
                        let class_obj = class
                            .to_object(context)?
                            .construct(
                                &[JsValue::String(js_string!(module_parent_path))],
                                None,
                                context,
                            )
                            .expect("construct error");

                        class_obj.set(
                            js_string!("typedId"),
                            JsValue::Integer(custom_typed_id),
                            false,
                            context,
                        )?;

                        let unit_proxy = JsProxy::from_object(
                            class_obj
                                .get(js_string!("getSynchronizeProxy"), context)?
                                .as_function()
                                .unwrap()
                                .call(&JsValue::Object(class_obj), &[], context)?
                                .to_object(context)?
                                .clone(),
                        )?;
                        let entity = JsEntity::try_from_js(
                            &unit_proxy.get(js_string!("entity"), context)?,
                            context,
                        )?
                        .to_entity();

                        emit_signal(
                            &unit_proxy
                                .get(js_string!("created"), context)?
                                .to_object(context)?,
                            &[],
                            context,
                        )?;

                        let section = Section::try_from_proxy(&unit_proxy, context)?;

                        context
                            .realm()
                            .host_defined_mut()
                            .get_mut::<UnitMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(entity, unit_proxy);

                        response_sender
                            .send(JsEngineResponseEvent::spawned_unit(SpawnedUnitData::new(
                                section,
                                unit_id,
                                entity,
                                module_path,
                                custom_typed_id,
                            )))
                            .unwrap();
                    }
                }
            }
        }
        JsEngineRequestEvent::ToTeleport(teleport_type) => match teleport_type {
            TeleportType::Position { this, position } => {
                response_sender
                    .send(JsEngineResponseEvent::ToTeleport(TeleportType::position(
                        this, position,
                    )))
                    .unwrap();
            }
            TeleportType::Entity { this, target } => {
                response_sender
                    .send(JsEngineResponseEvent::ToTeleport(TeleportType::entity(
                        this, target,
                    )))
                    .unwrap();
            }
        },
        JsEngineRequestEvent::SelectedSignalEmit => {
            let selected_signal_map = context
                .realm()
                .host_defined()
                .get::<SelectedSignalMap>()
                .unwrap()
                .map
                .clone();
            selected_signal_map.borrow().iter().for_each(|(_, signal)| {
                emit_signal(signal, &[], context).unwrap();
            });
        }
        JsEngineRequestEvent::ToLook(look_type) => match look_type {
            LookType::Position { this, position } => {
                response_sender
                    .send(JsEngineResponseEvent::ToLook(LookType::position(
                        this, position,
                    )))
                    .unwrap();
            }
            LookType::Entity { this, target } => {
                response_sender
                    .send(JsEngineResponseEvent::ToLook(LookType::entity(
                        this, target,
                    )))
                    .unwrap();
            }
        },
        JsEngineRequestEvent::OnUnitEnterSignal {
            target_entities,
            signal_entity,
        } => {
            let unit_enter_signal = context
                .realm()
                .host_defined()
                .get::<OnUnitEnterSignalMap>()
                .unwrap()
                .map
                .borrow()
                .get(&signal_entity)
                .unwrap()
                .clone();

            emit_signal(
                &unit_enter_signal,
                &[target_entities.try_into_js(context)?],
                context,
            )?;
        }
        JsEngineRequestEvent::OnUnitExitSignal {
            target_entities,
            signal_entity,
        } => {
            let unit_exit_signal = context
                .realm()
                .host_defined()
                .get::<OnUnitExitSignalMap>()
                .unwrap()
                .map
                .borrow()
                .get(&signal_entity)
                .unwrap()
                .clone();
            emit_signal(
                &unit_exit_signal,
                &[target_entities.try_into_js(context)?],
                context,
            )?;
        }
        JsEngineRequestEvent::EmitEmptySignal { signal_entity } => {
            let signal = context
                .realm()
                .host_defined()
                .get::<SignalEntityMap>()
                .unwrap()
                .map
                .borrow()
                .get(&signal_entity)
                .unwrap()
                .clone();
            emit_signal(&signal, &[], context)?;
        }
        JsEngineRequestEvent::SynchronizeData(synchronize_data) => match synchronize_data {
            SynchronizeData::Core(core) => {
                let core_object = context
                    .realm()
                    .host_defined()
                    .get::<JsObjectMap>()
                    .unwrap()
                    .map
                    .borrow()
                    .get(&core.entity)
                    .unwrap()
                    .clone();

                core_object
                    .get(js_string!("synchronize"), context)?
                    .as_function()
                    .unwrap()
                    .call(
                        &JsValue::Object(core_object),
                        &[core.try_into_js(context)?],
                        context,
                    )?;
            }
            SynchronizeData::Movement(movement) => {
                let movement_object = context
                    .realm()
                    .host_defined()
                    .get::<JsObjectMap>()
                    .unwrap()
                    .map
                    .borrow()
                    .get(&movement.entity)
                    .unwrap()
                    .clone();

                movement_object
                    .get(js_string!("synchronize"), context)?
                    .as_function()
                    .unwrap()
                    .call(
                        &JsValue::Object(movement_object),
                        &[movement.try_into_js(context)?],
                        context,
                    )?;
            }
        },
        JsEngineRequestEvent::InsertCustomInnerInfo {
            custom_typed_id,
            entity,
            custom_inner_info,
        } => {
            context
                .realm()
                .host_defined_mut()
                .get_mut::<CustomInnerInfoMap>()
                .unwrap()
                .map
                .borrow_mut()
                .entry(custom_typed_id)
                .or_insert((vec![entity], custom_inner_info))
                .0
                .push(entity);
        }
    }
    Ok(())
}

pub fn emit_signal(signal: &JsObject, args: &[JsValue], context: &mut Context) -> JsResult<()> {
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
