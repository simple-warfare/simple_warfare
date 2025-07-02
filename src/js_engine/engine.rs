use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
};

use bevy::{platform::collections::HashMap, prelude::*};
use boa_engine::{
    JsArgs, JsResult, js_string, object::builtins::JsProxy, prelude::*, property::Attribute,
};
use boa_runtime::Console;
use rustc_hash::FxHashMap;
use url::Url;

use crate::{
    assets::mods::js::JsAsset,
    js_engine::{
        event::{SwRequireLoaderRequestEvent, SwRequireLoaderResponseEvent},
        loader::SimpleWarfareModuleLoader,
        module::ModModule,
    },
    unit::section::core::Core,
};

pub struct JsEngine {
    pub(super) context: Context,
    pub(super) module_map: HashMap<String, Vec<ModModule>>,
    pub(super) unit_map: HashMap<Entity, JsProxy>,
    path_url: Arc<FxHashMap<&'static str, &'static str>>,
    request_sender: Arc<Sender<SwRequireLoaderRequestEvent>>,
    Response_receiver: Arc<Mutex<Receiver<SwRequireLoaderResponseEvent>>>,
}

impl JsEngine {
    pub fn new(
        loader: SimpleWarfareModuleLoader,
        request_sender: Arc<Sender<SwRequireLoaderRequestEvent>>,
        Response_receiver: Arc<Mutex<Receiver<SwRequireLoaderResponseEvent>>>,
    ) -> Self {
        let context_builder = Context::builder();
        let loader = Rc::new(loader);

        let mut context = context_builder
            .module_loader(loader.clone())
            .build()
            .expect("Build Js Context error!");

        let ctx = RefCell::new(&mut context);
        egister_global_property(&mut ctx.borrow_mut());
        register_global_class(&mut ctx.borrow_mut());

        let mut path_url = FxHashMap::default();
        path_url.insert("package", "mods/package/");

        let path_url = Arc::new(path_url);
        register_global_callable(
            &mut ctx.borrow_mut(),
            path_url.clone(),
            request_sender.clone(),
            Response_receiver.clone(),
        );

        Self {
            context,
            module_map: HashMap::new(),
            unit_map: HashMap::new(),
            path_url,
            request_sender,
            Response_receiver,
        }
    }
}

fn egister_global_property(ctx: &mut Context) {
    let console = Console::init(ctx);
    ctx.register_global_property(Console::NAME, console, Attribute::all())
        .expect("the console builtin shouldn't exist");
}

fn register_global_class(ctx: &mut Context) {}
pub fn get_real_path(
    path_url: Arc<FxHashMap<&'static str, &'static str>>,
    url: &Url,
) -> JsResult<String> {
    if let Some(path) = path_url.get(url.scheme()) {
        Ok(format!("{}{}", path, url.path()))
    } else {
        Err(JsError::from_opaque(
            js_string!(format!(
                "could not get the url's real path which called {}",
                url.scheme()
            ))
            .into(),
        ))
    }
}

fn register_global_callable(
    ctx: &mut Context,
    path_url: Arc<FxHashMap<&'static str, &'static str>>,
    request_sender: Arc<Sender<SwRequireLoaderRequestEvent>>,
    Response_receiver: Arc<Mutex<Receiver<SwRequireLoaderResponseEvent>>>,
) {
    let moduleobj = JsObject::default();
    moduleobj
        .set(
            js_string!("exports"),
            JsValue::from(js_string!(" ")),
            false,
            ctx,
        )
        .unwrap();

    ctx.register_global_property(
        js_string!("module"),
        JsValue::from(moduleobj),
        Attribute::default(),
    )
    .unwrap();

    ctx.register_global_callable("require".into(), 0, unsafe {
        NativeFunction::from_closure(move |referrer, args, ctx| {
            let arg = args.get_or_undefined(0);
            let lib_file = arg.to_string(ctx)?.to_std_string_escaped();
            let url = Url::parse(&lib_file).map_err(|err| {
                JsNativeError::typ()
                    .with_message(format!("could not parse url `{lib_file}`"))
                    .with_cause(JsError::from_opaque(js_string!(err.to_string()).into()))
            })?;
            let real_path = get_real_path(path_url.clone(), &url)?;
            request_sender
                .send(SwRequireLoaderRequestEvent::LoadJsAsset(real_path.clone()))
                .or(Err(JsError::from_opaque(
                    js_string!(format!("request_sender could not send `{real_path}`")).into(),
                )))?;
            // Read the module source file
            println!("Loading: {real_path}");
            let Response_receiver = Response_receiver.clone();
            let js_asset = match Response_receiver
                .lock()
                .map_err(|err| {
                    JsNativeError::typ()
                        .with_message(format!(
                            "could lock the Response receiver when load `{real_path}`"
                        ))
                        .with_cause(JsError::from_opaque(JsValue::String(js_string!(
                            err.to_string()
                        ))))
                })?
                .recv()
            {
                Ok(event) => match event {
                    SwRequireLoaderResponseEvent::LoadedJsAsset(js_asset) => {
                        Ok::<JsAsset, JsError>(js_asset)
                    }
                },
                Err(err) => Err(JsNativeError::typ()
                    .with_message(format!(
                        "could lock the Response receiver when load `{real_path}`"
                    ))
                    .with_cause(JsError::from_opaque(JsValue::String(js_string!(
                        err.to_string()
                    ))))
                    .into()),
            }?;

            ctx.eval(Source::from_bytes(&js_asset.context))?;

            // Access module.exports and return as ResultValue
            let global_obj = ctx.global_object();
            let module = global_obj.get(js_string!("module"), ctx)?;
            module
                .as_object()
                .ok_or_else(|| {
                    JsNativeError::typ().with_message("`exports` property was not an object")
                })?
                .get(js_string!("exports"), ctx)
        })
    })
    .unwrap();
}
