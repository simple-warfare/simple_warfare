pub mod fs;
pub mod plugin;
use bevy::prelude::*;
use boa_engine::{
    JsArgs, JsResult, js_string,
    object::{ObjectInitializer, builtins::JsArray},
    prelude::*,
    property::Attribute,
    value::{TryFromJs, TryIntoJs},
};
use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, Sender},
};

use crate::{
    bevy_ext::try_from_js::vec2_try_from_js,
    custom::ui::quick::QuickUi,
    js_engine::{
        context::emit_signal, event::JsEngineRequestEvent, global::class::entity::JsEntity,
        host_defined::*, signal::JsDefaultSignalType, sw::fs::Fs,
    },
};

#[derive(Resource)]
pub struct SwRequestReceiver(pub Arc<Mutex<Receiver<SwRequestEvent>>>);

#[derive(Resource, Clone)]
pub struct SwResponseSender(pub Arc<Sender<SwResponseEvent>>);

#[derive(Debug, Default, Trace, Finalize, JsData)]
pub struct Sw;

#[derive(Event)]
pub enum SwRequestEvent {
    RegisterEntity,
    ReadFile(Box<oneshot::Sender<String>>, String),
    CreateQuickUi(QuickUi),
}

#[derive(Event, Clone)]
pub enum SwResponseEvent {
    None,
    RegisteredEntity(Entity),
}

#[derive(Debug, Clone, Copy)]
pub enum TeleportType {
    Position(JsEntity, Vec2),
    Entity(JsEntity, JsEntity),
}

#[derive(Debug, Clone, Copy)]
pub enum LookType {
    Position(JsEntity, Vec2),
    Entity(JsEntity, JsEntity),
}

#[derive(Debug, Clone, Copy)]
pub enum JsTargetType {
    Position,
    Entity,
}

impl TryFromJs for JsTargetType {
    fn try_from_js(value: &JsValue, _context: &mut Context) -> JsResult<Self> {
        match value {
            JsValue::String(target_type) => match target_type.to_std_string_lossy().as_str() {
                "Position" => Ok(JsTargetType::Position),
                "Entity" => Ok(JsTargetType::Entity),
                _ => Err(JsNativeError::typ()
                    .with_message("cannot convert value to a JsTeleportType")
                    .into()),
            },
            _ => Err(JsNativeError::typ()
                .with_message("cannot convert value to a JsTeleportType")
                .into()),
        }
    }
}
impl Sw {
    pub const NAME: JsString = js_string!("sw");

    pub fn init(
        context: &mut Context,
        js_engine_request_sender: Arc<Sender<JsEngineRequestEvent>>,
        sw_request_sender: Arc<Sender<SwRequestEvent>>,
        sw_response_receiver: Arc<Mutex<Receiver<SwResponseEvent>>>,
    ) -> JsObject {
        let teleport = unsafe {
            let js_engine_request_sender = js_engine_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let js_teleport_type = JsTargetType::try_from_js(args.first().unwrap(), ctx)?;
                match js_teleport_type {
                    JsTargetType::Position => js_engine_request_sender
                        .send(JsEngineRequestEvent::ToTeleport(TeleportType::Position(
                            JsEntity::try_from_js(args.get(1).unwrap(), ctx)?,
                            vec2_try_from_js(args.get(2).unwrap(), ctx)?,
                        )))
                        .unwrap(),

                    JsTargetType::Entity => js_engine_request_sender
                        .send(JsEngineRequestEvent::ToLook(LookType::Entity(
                            JsEntity::try_from_js(args.get(1).unwrap(), ctx)?,
                            JsEntity::try_from_js(args.get(2).unwrap(), ctx)?,
                        )))
                        .unwrap(),
                }
                Ok(JsValue::undefined())
            })
        };

        let look_at = unsafe {
            let js_engine_request_sender = js_engine_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let js_look_type = JsTargetType::try_from_js(args.first().unwrap(), ctx)?;
                match js_look_type {
                    JsTargetType::Position => js_engine_request_sender
                        .send(JsEngineRequestEvent::ToLook(LookType::Position(
                            JsEntity::try_from_js(args.get(1).unwrap(), ctx)?,
                            vec2_try_from_js(args.get(2).unwrap(), ctx)?,
                        )))
                        .unwrap(),

                    JsTargetType::Entity => js_engine_request_sender
                        .send(JsEngineRequestEvent::ToLook(LookType::Entity(
                            JsEntity::try_from_js(args.get(1).unwrap(), ctx)?,
                            JsEntity::try_from_js(args.get(2).unwrap(), ctx)?,
                        )))
                        .unwrap(),
                }
                Ok(JsValue::undefined())
            })
        };

        let signal_emit = unsafe {
            NativeFunction::from_closure(|_referrer, args, ctx| {
                let signal = args.first().unwrap().to_object(ctx)?;

                let signal_args = args[1..].to_owned();
                let connect_array = JsArray::from_object(
                    signal
                        .get(js_string!("connectArray"), ctx)?
                        .to_object(ctx)?,
                )?;

                let func = connect_array
                    .get(js_string!("0"), ctx)?
                    .as_function()
                    .unwrap();
                func.call(&JsValue::Undefined, &signal_args, ctx)?;
                Ok(JsValue::undefined())
            })
        };

        let register_default_signal = unsafe {
            NativeFunction::from_closure(|_referrer, args, ctx| {
                let signal = args.first().unwrap().to_object(ctx)?;

                let default_signal_type =
                    JsDefaultSignalType::try_from_js(&signal.get(js_string!("type"), ctx)?, ctx)?;

                match default_signal_type {
                    JsDefaultSignalType::Created => emit_signal(&signal, &[], ctx)?,
                    JsDefaultSignalType::Selected => {
                        let js_entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?;
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<SelectedSignalMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(js_entity, signal);
                    }
                    JsDefaultSignalType::OnUnitEnter => {
                        let js_entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?;
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<OnUnitEnterSignalMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(js_entity, signal);
                    }
                    JsDefaultSignalType::OnUnitExit => {
                        let js_entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?;
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<OnUnitExitSignalMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(js_entity, signal);
                    }
                }

                Ok(JsValue::undefined())
            })
        };

        let register_entity = unsafe {
            let sw_request_sender = sw_request_sender.clone();
            let sw_response_receiver = sw_response_receiver.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                sw_request_sender
                    .send(SwRequestEvent::RegisterEntity)
                    .unwrap();
                if let SwResponseEvent::RegisteredEntity(entity) =
                    sw_response_receiver.lock().unwrap().recv().unwrap()
                {
                    let js_entity = JsEntity::from_entity(&entity);
                    let js_object = args.get_or_undefined(0).to_object(ctx)?;
                    ctx.realm()
                        .host_defined_mut()
                        .get_mut::<EntityMap>()
                        .unwrap()
                        .map
                        .borrow_mut()
                        .insert(js_entity, entity);
                    ctx.realm()
                        .host_defined_mut()
                        .get_mut::<JsObjectMap>()
                        .unwrap()
                        .map
                        .borrow_mut()
                        .insert(js_entity, js_object);
                    Ok(JsValue::Object(js_entity.try_into_js(ctx)?.to_object(ctx)?))
                } else {
                    Ok(JsValue::undefined())
                }
            })
        };

        let create_quick_ui = unsafe {
            let sw_request_sender = sw_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let quick_ui = args.first().unwrap();
                sw_request_sender
                    .send(SwRequestEvent::CreateQuickUi(QuickUi::try_from_js(
                        quick_ui, ctx,
                    )?))
                    .unwrap();
                Ok(JsValue::Undefined)
            })
        };

        let register_signal = unsafe {
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let signal_object = args.get_or_undefined(0).to_object(ctx)?;
                let js_entity =
                    JsEntity::try_from_js(&signal_object.get(js_string!("entity"), ctx)?, ctx)?;
                ctx.realm()
                    .host_defined_mut()
                    .get_mut::<SignalEntityMap>()
                    .unwrap()
                    .map
                    .borrow_mut()
                    .insert(js_entity, signal_object);
                Ok(JsValue::Undefined)
            })
        };

        let get_object = unsafe {
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let target_entity = JsEntity::try_from_js(args.get_or_undefined(0), ctx)?;
                let target_object = match ctx
                    .realm()
                    .host_defined_mut()
                    .get::<JsObjectMap>()
                    .unwrap()
                    .map
                    .borrow()
                    .get(&target_entity)
                {
                    Some(object) => JsValue::Object(object.clone()),
                    None => JsValue::Undefined,
                };
                Ok(target_object)
            })
        };

        let fs = Fs::init(
            context,
            js_engine_request_sender.clone(),
            sw_request_sender.clone(),
            sw_response_receiver.clone(),
        );

        ObjectInitializer::with_native_data_and_proto(
            Self::default(),
            JsObject::with_object_proto(context.realm().intrinsics()),
            context,
        )
        .property(
            JsSymbol::to_string_tag(),
            Self::NAME,
            Attribute::CONFIGURABLE,
        )
        .function(teleport, js_string!("teleport"), 3)
        .function(look_at, js_string!("lookAt"), 3)
        //.function(register_signal, js_string!("register_signal"), 1)
        .function(signal_emit, js_string!("signal_emit"), 2)
        .function(register_entity, js_string!("register_entity"), 0)
        .function(
            register_default_signal,
            js_string!("registerDefaultSignal"),
            1,
        )
        .function(create_quick_ui, js_string!("create_quick_ui"), 1)
        .function(register_signal, js_string!("register_signal"), 1)
        .function(get_object, js_string!("getObject"), 1)
        .property(js_string!("fs"), fs, Attribute::CONFIGURABLE)
        .build()
    }
}
