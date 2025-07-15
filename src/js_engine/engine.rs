use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
};

use bevy::{platform::collections::HashMap, prelude::*};
use boa_engine::{JsArgs, JsResult, js_string, prelude::*, property::Attribute};
use boa_runtime::Console;
use rustc_hash::FxHashMap;
use url::Url;

use crate::{
    assets::mods::js::JsAsset,
    js_engine::{
        event::{JsEngineRequestEvent, SwRequireLoaderRequestEvent, SwRequireLoaderResponseEvent},
        host_defined::*,
        loader::SimpleWarfareModuleLoader,
        module::ModModule,
        sw::{Sw, SwRequestEvent, SwResponseEvent},
    },
};

pub struct JsEngine {
    pub(super) context: Context,
    pub(super) module_map: HashMap<String, Vec<ModModule>>,
}

impl JsEngine {
    pub fn new(
        loader: SimpleWarfareModuleLoader,
        js_engine_request_sender: Arc<Sender<JsEngineRequestEvent>>,
        require_request_sender: Arc<Sender<SwRequireLoaderRequestEvent>>,
        require_response_receiver: Arc<Mutex<Receiver<SwRequireLoaderResponseEvent>>>,
        sw_request_sender: Arc<Sender<SwRequestEvent>>,
        sw_response_receiver: Arc<Mutex<Receiver<SwResponseEvent>>>,
    ) -> Self {
        let context_builder = Context::builder();
        let loader = Rc::new(loader);

        let mut context = context_builder
            .module_loader(loader.clone())
            .build()
            .expect("Build Js Context error!");

        let ctx = RefCell::new(&mut context);

        insert_host_defined_data(&mut ctx.borrow_mut());
        egister_global_property(
            &mut ctx.borrow_mut(),
            js_engine_request_sender,
            sw_request_sender,
            sw_response_receiver,
        );
        register_global_class(&mut ctx.borrow_mut());

        let mut path_url = FxHashMap::default();
        path_url.insert("package", "mods/package/");

        let path_url = Arc::new(path_url);
        register_global_callable(
            &mut ctx.borrow_mut(),
            path_url.clone(),
            require_request_sender.clone(),
            require_response_receiver.clone(),
        );

        Self {
            context,
            module_map: HashMap::new(),
        }
    }
}

fn egister_global_property(
    ctx: &mut Context,
    js_engine_request_sender: Arc<Sender<JsEngineRequestEvent>>,
    sw_request_sender: Arc<Sender<SwRequestEvent>>,
    sw_response_receiver: Arc<Mutex<Receiver<SwResponseEvent>>>,
) {
    let console = Console::init(ctx);
    ctx.register_global_property(Console::NAME, console, Attribute::all())
        .expect("the console builtin shouldn't exist");

    let sw = Sw::init(
        ctx,
        js_engine_request_sender,
        sw_request_sender,
        sw_response_receiver,
    );

    ctx.register_global_property(Sw::NAME, sw, Attribute::all())
        .expect("the sw builtin shouldn't exist");
}

fn register_global_class(_ctx: &mut Context) {}
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
    response_receiver: Arc<Mutex<Receiver<SwRequireLoaderResponseEvent>>>,
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
        NativeFunction::from_closure(move |_referrer, args, ctx| {
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
            let response_receiver = response_receiver.clone();
            let js_asset = match response_receiver
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

fn insert_host_defined_data(ctx: &mut Context) {
    ctx.realm().host_defined_mut().insert(UnitMap::default());
    ctx.realm().host_defined_mut().insert(EntityMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(SelectedSignalMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(OnUnitEnterSignalMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(OnUnitExitSignalMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(SignalEntityMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(JsObjectMap::default());
}
