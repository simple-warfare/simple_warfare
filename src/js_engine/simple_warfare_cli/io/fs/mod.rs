pub mod plugin;

use bevy::prelude::*;
use boa_engine::{
    js_string, object::ObjectInitializer, prelude::*, property::Attribute, value::TryIntoJs,
};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
};

use crate::assets::js_file::{section::SectionFile, toml::TomlFile};

/// Js端Sw的成员之一，负责实现Js加载文件
#[derive(Debug, Default, Trace, Finalize, JsData)]
pub struct Fs;

#[derive(Resource)]
pub struct SwFsRequestReceiver(pub Arc<Mutex<Receiver<SwFsRequestEvent>>>);

#[derive(Resource, Clone)]
pub struct SwFsResponseSender(pub Arc<Sender<SwFsResponseEvent>>);

#[derive(Event)]
pub enum SwFsRequestEvent {
    ReadSectionFile {
        file_sender: Box<oneshot::Sender<SectionFile>>,
        file_path: PathBuf,
    },
    ReadTomlFile {
        file_sender: Box<oneshot::Sender<TomlFile>>,
        file_path: PathBuf,
    },
}

impl SwFsRequestEvent {
    pub fn read_section_file(
        file_sender: Box<oneshot::Sender<SectionFile>>,
        file_path: PathBuf,
    ) -> Self {
        SwFsRequestEvent::ReadSectionFile {
            file_sender,
            file_path,
        }
    }
    pub fn read_toml_file(file_sender: Box<oneshot::Sender<TomlFile>>, file_path: PathBuf) -> Self {
        SwFsRequestEvent::ReadTomlFile {
            file_sender,
            file_path,
        }
    }
}

#[derive(Event)]
pub enum SwFsResponseEvent {}

impl Fs {
    pub const NAME: JsString = js_string!("fs");

    /// TODO:目前只能加载String到Js端,等待完善
    pub fn init(
        context: &mut Context,
        sw_fs_request_sender: Arc<Sender<SwFsRequestEvent>>,
    ) -> JsObject {
        let read_section_file = unsafe {
            let sw_fs_request_sender = sw_fs_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                // Js入参中第二个应该为文件路径
                let Some(this) = args.first() else {
                    return Ok(JsValue::Boolean(false));
                };
                let Some(small_path) = args.get(1) else {
                    return Ok(JsValue::Boolean(false));
                };

                let this_object = this.to_object(ctx)?;
                let module_path = this_object
                    .get(js_string!("moduleParentPath"), ctx)?
                    .to_string(ctx)?
                    .to_std_string_lossy();

                let module_path = Path::new(&module_path);

                let real_path = module_path.join(small_path.to_string(ctx)?.to_std_string_lossy());

                // 一次性管道用于接受加载好的文件
                let (sender, receiver) = oneshot::channel();
                sw_fs_request_sender
                    .send(SwFsRequestEvent::read_section_file(
                        Box::new(sender),
                        real_path,
                    ))
                    .unwrap();
                if let Ok(section_file) = receiver.recv() {
                    let section_object = section_file.try_into_js(ctx)?.to_object(ctx)?;

                    Ok(JsValue::Object(section_object))
                } else {
                    Ok(JsValue::undefined())
                }
            })
        };

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
        .function(read_section_file, js_string!("readSectionFile"), 2)
        .build()
    }
}
