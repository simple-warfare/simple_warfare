pub mod plugin;
use bevy::prelude::*;
use boa_engine::{
    JsResult, js_string,
    object::{ObjectInitializer, builtins::JsArray},
    prelude::*,
    property::Attribute,
    value::TryFromJs,
};
use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, Sender},
};

use crate::{bevy_ext::try_from_js::vec2_try_from_js, js_engine::global::class::entity::JsEntity};

#[derive(Resource)]
pub struct SwRequestReceiver(pub Arc<Mutex<Receiver<SwRequestEvent>>>);

#[derive(Resource, Clone)]
pub struct SwResponseSender(pub Arc<Sender<SwResponseEvent>>);

#[derive(Debug, Default, Trace, Finalize, JsData)]
pub struct Sw;

#[derive(Event, Clone)]
pub enum SwRequestEvent {
    Teleport(TeleportType),
}

#[derive(Event, Clone)]
pub enum SwResponseEvent {
    Teleported(TeleportType),
}

#[derive(Debug, Clone, Copy)]
pub enum TeleportType {
    Position(JsEntity, Vec2),
}

#[derive(Debug, Clone, Copy)]
pub enum JsTeleportType {
    Position,
}

impl TryFromJs for JsTeleportType {
    fn try_from_js(value: &JsValue, _context: &mut Context) -> JsResult<Self> {
        match value {
            JsValue::String(teleport_type) => match teleport_type.to_std_string_lossy().as_str() {
                "Position" => Ok(JsTeleportType::Position),
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
        sw_request_sender: Arc<Sender<SwRequestEvent>>,
        _sw_response_receiver: Arc<Mutex<Receiver<SwResponseEvent>>>,
    ) -> JsObject {
        let teleport = unsafe {
            let sw_request_sender = sw_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let js_teleport_type = JsTeleportType::try_from_js(args.first().unwrap(), ctx)?;
                match js_teleport_type {
                    JsTeleportType::Position => {
                        let target = vec2_try_from_js(args.get(2).unwrap(), ctx)?;
                        sw_request_sender
                            .send(SwRequestEvent::Teleport(TeleportType::Position(
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
        //.function(register_signal, js_string!("register_signal"), 1)
        .function(signal_emit, js_string!("signal_emit"), 2)
        .build()
    }
}
