pub mod plugin;
use bevy::prelude::*;
use boa_engine::{
    JsArgs, JsResult, js_string, object::ObjectInitializer, prelude::*, property::Attribute,
    value::TryFromJs,
};
use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, Sender},
};

use crate::{
    bevy_ext::try_from_js::try_from_js_to_vec2,
    js_engine::{
        event::SafetyJsValue,
        global::class::entity::JsEntity,
        signal::{EmitSignal, HostDefinedSignalSystem},
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
    Teleport(TeleportType),
    EmitSignal,
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
                        let target = try_from_js_to_vec2(args.get(2).unwrap(), ctx)?;
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
        /*
               let register_signal = unsafe {
                   NativeFunction::from_closure(|_referrer, args, ctx| {
                       let signal = args.first().unwrap().to_object(ctx)?;
                       let signal_name = signal.get(js_string!("name"), ctx)?.to_string(ctx)?;
                       ctx.realm()
                           .host_defined_mut()
                           .get_mut::<HostDefinedSignalSystem>()
                           .unwrap()
                           .signal_map
                           .insert(signal_name.clone(), signal);
                       Ok(JsValue::undefined())
                   })
               };
        */
        let signal_emit = unsafe {
            let sw_request_sender = sw_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let signal = args.first().unwrap().to_object(ctx)?;

                let signal_args = args[1..].to_owned();
                info!("{:?}", signal_args);
                ctx.realm()
                    .host_defined_mut()
                    .get_mut::<HostDefinedSignalSystem>()
                    .unwrap()
                    .insert_emit_signal(EmitSignal::new(signal, signal_args));

                sw_request_sender.send(SwRequestEvent::EmitSignal).unwrap();
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
