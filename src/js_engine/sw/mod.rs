pub mod plugin;
use bevy::prelude::*;
use boa_engine::{
    JsResult, js_string,
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
    js_engine::{
        context::emit_signal, event::JsEngineRequestEvent, global::class::entity::JsEntity,
        host_defined::*, signal::JsDefaultSignalType,
    },
};

#[derive(Resource)]
pub struct SwRequestReceiver(pub Arc<Mutex<Receiver<SwRequestEvent>>>);

#[derive(Resource, Clone)]
pub struct SwResponseSender(pub Arc<Sender<SwResponseEvent>>);

#[derive(Debug, Default, Trace, Finalize, JsData)]
pub struct Sw;

#[derive(Event, Clone)]
pub enum SwRequestEvent {
    RegisterEntity,
}

#[derive(Event, Clone)]
pub enum SwResponseEvent {
    None,
    RegisteredEntity(Entity),
}

#[derive(Debug, Clone, Copy)]
pub enum TeleportType {
    Position(JsEntity, Vec2),
}

#[derive(Debug, Clone, Copy)]
pub enum LookType {
    Position(JsEntity, Vec2),
}

#[derive(Debug, Clone, Copy)]
pub enum JsTargetType {
    Position,
}

impl TryFromJs for JsTargetType {
    fn try_from_js(value: &JsValue, _context: &mut Context) -> JsResult<Self> {
        match value {
            JsValue::String(target_type) => match target_type.to_std_string_lossy().as_str() {
                "Position" => Ok(JsTargetType::Position),
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
                    JsTargetType::Position => {
                        let target = vec2_try_from_js(args.get(2).unwrap(), ctx)?;
                        js_engine_request_sender
                            .send(JsEngineRequestEvent::ToTeleport(TeleportType::Position(
                                JsEntity::try_from_js(args.get(1).unwrap(), ctx)?,
                                target,
                            )))
                            .unwrap();
                    }
                }
                Ok(JsValue::undefined())
            })
        };

        let look_at = unsafe {
            let js_engine_request_sender = js_engine_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let js_look_type = JsTargetType::try_from_js(args.first().unwrap(), ctx)?;
                match js_look_type {
                    JsTargetType::Position => {
                        let target = vec2_try_from_js(args.get(2).unwrap(), ctx)?;
                        js_engine_request_sender
                            .send(JsEngineRequestEvent::ToLook(LookType::Position(
                                JsEntity::try_from_js(args.get(1).unwrap(), ctx)?,
                                target,
                            )))
                            .unwrap();
                    }
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
                    JsDefaultSignalType::UnitEnter => {
                        let js_entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?;
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<UnitEnterSignalMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(js_entity, signal);
                    }
                    JsDefaultSignalType::UnitExit => {
                        let js_entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?;
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<UnitExitSignalMap>()
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
            NativeFunction::from_closure(move |_referrer, _args, ctx| {
                sw_request_sender
                    .send(SwRequestEvent::RegisterEntity)
                    .unwrap();
                if let SwResponseEvent::RegisteredEntity(entity) =
                    sw_response_receiver.lock().unwrap().recv().unwrap()
                {
                    let js_entity = JsEntity::from_entity(&entity);
                    ctx.realm()
                        .host_defined_mut()
                        .get_mut::<EntityMap>()
                        .unwrap()
                        .map
                        .borrow_mut()
                        .insert(js_entity, entity);
                    Ok(JsValue::Object(js_entity.try_into_js(ctx)?.to_object(ctx)?))
                } else {
                    Ok(JsValue::undefined())
                }
            })
        };

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
        .build()
    }
}
