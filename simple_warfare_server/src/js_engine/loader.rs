use std::{
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
};

use crate::{assets::mods::js::JsAsset, js_engine::event::SwModuleLoaderRequestEvent};
use bevy::prelude::*;
use boa_engine::{
    JsResult, js_string,
    module::{ModuleLoader, Referrer},
    prelude::*,
};
use boa_gc::GcRefCell;
use boa_profiler::Profiler;
use rustc_hash::FxHashMap;
use std::fmt::Debug;

use url::Url;

#[derive(Resource)]
pub struct SwModuleLoaderRequestReceiver(pub Arc<Mutex<Receiver<SwModuleLoaderRequestEvent>>>);

#[derive(Debug)]
pub struct SimpleWarfareModuleLoader {
    _root: PathBuf,
    module_map: GcRefCell<FxHashMap<String, Module>>,
    path_url: Arc<FxHashMap<&'static str, &'static str>>,
    request_sender: Arc<Sender<SwModuleLoaderRequestEvent>>,
}

impl SimpleWarfareModuleLoader {
    pub fn new<P: AsRef<Path>>(
        root: P,
        request_sender: Arc<Sender<SwModuleLoaderRequestEvent>>,
    ) -> JsResult<Self> {
        Profiler::global().start_event("Loader::new", "Loader");
        if cfg!(target_family = "wasm") {
            return Err(JsNativeError::typ()
                .with_message("cannot resolve a relative path in WASM targets")
                .into());
        }
        let root = root.as_ref();
        let absolute = root.canonicalize().map_err(|e| {
            JsNativeError::typ()
                .with_message(format!("could not set module root `{}`", root.display()))
                .with_cause(JsError::from_opaque(js_string!(e.to_string()).into()))
        })?;
        let mut path_url = FxHashMap::default();
        path_url.insert("std", "mods/std/");
        path_url.insert("package", "mods/package/");
        path_url.insert("custom", "mods/custom/");
        Ok(Self {
            _root: absolute,
            module_map: GcRefCell::default(),
            path_url: Arc::new(path_url),
            request_sender,
        })
    }

    #[inline]
    pub fn insert(&self, url: &str, module: Module) {
        self.module_map.borrow_mut().insert(url.to_string(), module);
    }

    #[inline]
    pub fn get(&self, url: &str) -> Option<Module> {
        self.module_map.borrow().get(url).cloned()
    }
    #[inline]
    pub fn get_real_path(&self, url: &Url) -> JsResult<String> {
        if let Some(path) = self.path_url.get(url.scheme()) {
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

    pub fn load(&self, real_path: String) -> JsResult<JsAsset> {
        let (sender, receiver) = oneshot::channel();
        self.request_sender
            .send(SwModuleLoaderRequestEvent::load_js_asset(
                real_path.clone(),
                Box::new(sender),
            ))
            .or(Err(JsError::from_opaque(
                js_string!(format!("request_sender could not send `{real_path}`")).into(),
            )))?;

        match receiver.recv() {
            Ok(js_asset) => Ok(js_asset),
            Err(err) => Err(JsNativeError::typ()
                .with_message(format!(
                    "could lock the Response receiver when load `{real_path}`"
                ))
                .with_cause(JsError::from_opaque(JsValue::String(js_string!(
                    err.to_string()
                ))))
                .into()),
        }
    }
}

impl ModuleLoader for SimpleWarfareModuleLoader {
    fn load_imported_module(
        &self,
        _referrer: Referrer,
        specifier: JsString,
        finish_load: Box<dyn FnOnce(JsResult<Module>, &mut Context)>,
        context: &mut Context,
    ) {
        let result = (|| {
            let specifier = specifier.to_std_string_escaped();

            let specifier_url = Url::parse(&specifier).map_err(|err| {
                JsNativeError::typ()
                    .with_message(format!("could not parse url `{specifier}`"))
                    .with_cause(JsError::from_opaque(js_string!(err.to_string()).into()))
            })?;

            if let Some(module) = self.get(&specifier) {
                return Ok(module);
            }

            let real_path = self.get_real_path(&specifier_url)?;
            let js_asset = self.load(real_path)?;

            let source = Source::from_bytes(&js_asset.context);
            let module = Module::parse(source, None, context)?;
            self.insert(&specifier, module.clone());
            Ok(module)
        })();

        finish_load(result, context);
    }

    fn register_module(&self, specifier: JsString, module: Module) {
        self.insert(&specifier.to_std_string_escaped(), module);
    }

    fn get_module(&self, specifier: JsString) -> Option<Module> {
        self.get(&specifier.to_std_string_escaped())
    }
}
