use std::{
    path::Path,
    sync::{Arc, mpsc::Sender},
};

use boa_engine::{
    js_string, object::ObjectInitializer, prelude::*, property::Attribute, value::TryIntoJs,
};

use crate::js_engine::simple_warfare_cli::SwRequestEvent;

/// Js端Sw的成员之一，负责实现Js加载文件
#[derive(Debug, Default, Trace, Finalize, JsData)]
pub struct TrickFilmPlayerServer;

impl TrickFilmPlayerServer {
    pub const NAME: JsString = js_string!("trickFilmPlayerServer");

    /// TODO:目前只能加载String到Js端,等待完善
    pub fn init(context: &mut Context, sw_request_sender: Arc<Sender<SwRequestEvent>>) -> JsObject {
        //创建fs这个Object
        ObjectInitializer::with_native_data_and_proto(
            Self,
            JsObject::with_object_proto(context.realm().intrinsics()),
            context,
        )
        .property(
            JsSymbol::to_string_tag(),
            Self::NAME,
            Attribute::CONFIGURABLE,
        )
        .build()
    }
}
