use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, Sender},
};

use boa_engine::{js_string, object::ObjectInitializer, prelude::*, property::Attribute};

use crate::js_engine::{
    event::JsEngineRequestEvent,
    sw::{SwRequestEvent, SwResponseEvent},
};

#[derive(Debug, Default, Trace, Finalize, JsData)]
pub struct Fs;

impl Fs {
    pub const NAME: JsString = js_string!("fs");

    pub fn init(
        context: &mut Context,
        js_engine_request_sender: Arc<Sender<JsEngineRequestEvent>>,
        sw_request_sender: Arc<Sender<SwRequestEvent>>,
        sw_response_receiver: Arc<Mutex<Receiver<SwResponseEvent>>>,
    ) -> JsObject {
        let read_file = unsafe {
            let sw_request_sender = sw_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let Some(path_str) = args.first() else {
                    return Ok(JsValue::undefined());
                };
                let (sender, receiver) = oneshot::channel();
                sw_request_sender
                    .send(SwRequestEvent::ReadFile(
                        Box::new(sender),
                        path_str.to_string(ctx)?.to_std_string_lossy(),
                    ))
                    .unwrap();
                if let Ok(string) =
                    receiver.recv()
                {
                    Ok(JsValue::String(js_string!(string)))
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
        .function(read_file, js_string!("readFile"), 2)
        .build()
    }
}
