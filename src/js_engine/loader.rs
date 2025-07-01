use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    pin::Pin,
    rc::Rc,
    sync::Arc,
};

use crate::assets::mods::js::JsAsset;
use bevy::{
    prelude::*,
    tasks::{
        AsyncComputeTaskPool,
        futures_lite::{StreamExt, future},
    },
};
use bevy_inspector_egui::egui::ahash::HashMapExt;
use boa_engine::{
    JsResult,
    job::{FutureJob, JobQueue, NativeJob},
    js_string,
    module::{ModuleLoader, Referrer},
    prelude::*,
};
use boa_gc::GcRefCell;
use boa_profiler::Profiler;
use futures_util::stream::FuturesUnordered;
use rustc_hash::FxHashMap;
use std::fmt::Debug;
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    mpsc::{self, UnboundedReceiver},
};

use url::Url;
#[derive(Debug, Event, Clone)]
pub enum SwModuleLoaderRequestEvent {
    LoadJsAsset(String),
}
#[derive(Debug, Event, Clone)]
pub enum SwModuleLoaderResponeEvent {
    LoadedJsAsset(JsAsset),
}

#[derive(Resource)]
pub struct SwModuleLoaderRequestReceiver(pub UnboundedReceiver<SwModuleLoaderRequestEvent>);

#[derive(Resource, Clone)]
pub struct SwModuleLoaderResponeSender(pub Arc<Sender<SwModuleLoaderResponeEvent>>);

#[derive(Debug)]
pub struct SimpleWarfareModuleLoader {
    module_map: GcRefCell<FxHashMap<String, Module>>,
    path_url: FxHashMap<&'static str, &'static str>,
    request_sender: Arc<mpsc::UnboundedSender<SwModuleLoaderRequestEvent>>,
    respone_sender: broadcast::Sender<SwModuleLoaderResponeEvent>,
}

impl SimpleWarfareModuleLoader {
    pub fn new(
        request_sender: Arc<mpsc::UnboundedSender<SwModuleLoaderRequestEvent>>,
        respone_sender: broadcast::Sender<SwModuleLoaderResponeEvent>,
    ) -> JsResult<Self> {
        let _timer = Profiler::global().start_event("Loader::new", "Loader");
        if cfg!(target_family = "wasm") {
            return Err(JsNativeError::typ()
                .with_message("cannot resolve a relative path in WASM targets")
                .into());
        }

        let mut path_url = FxHashMap::new();
        path_url.insert("std", "mods/std/");

        Ok(Self {
            module_map: GcRefCell::default(),
            path_url,
            request_sender,
            respone_sender,
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
}
pub async fn load(
    sender: Arc<mpsc::UnboundedSender<SwModuleLoaderRequestEvent>>,
    mut receiver: broadcast::Receiver<SwModuleLoaderResponeEvent>,
    real_path: String,
) -> JsResult<JsAsset> {
    sender
        .send(SwModuleLoaderRequestEvent::LoadJsAsset(real_path.clone()))
        .or(Err(JsError::from_opaque(
            js_string!(format!("could not send")).into(),
        )))?;

    info!("real_path:{:?}", real_path);

    match receiver.recv().await {
        Ok(event) => match event {
            SwModuleLoaderResponeEvent::LoadedJsAsset(js_asset) => return Ok(js_asset),
        },
        Err(err) => match err {
            _ => panic!("error"),
        },
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
        let specifier = specifier.to_std_string_escaped();
        let specifier_url = Url::parse(&specifier).expect("url parse error");

        if let Some(module) = self.get(&specifier) {
            finish_load(Ok(module), context);
        } else {
            let real_path = self.get_real_path(&specifier_url).unwrap();
            let request_sender = self.request_sender.clone();
            let respone_sender = self.respone_sender.subscribe();
            let fetch = async move {
                let js_asset = load(request_sender, respone_sender, real_path).await;
                NativeJob::new(move |context| -> JsResult<JsValue> {
                    let js_asset = match js_asset {
                        Ok(js_asset) => js_asset,
                        Err(err) => {
                            finish_load(
                                Err(JsNativeError::typ().with_message(err.to_string()).into()),
                                context,
                            );
                            return Ok(JsValue::undefined());
                        }
                    };
                    info!("js_asset:{:?}", js_asset.context);
                    let source = Source::from_bytes(&js_asset.context);
                    let module = Module::parse(source, None, context);
                    finish_load(module, context);
                    Ok(JsValue::undefined())
                })
            };
            context
                .job_queue()
                .enqueue_future_job(Box::pin(fetch), context)
        }
    }

    fn register_module(&self, specifier: JsString, module: Module) {
        self.insert(&specifier.to_std_string_escaped(), module);
    }

    fn get_module(&self, specifier: JsString) -> Option<Module> {
        self.get(&specifier.to_std_string_escaped())
    }
}

pub struct SwModuleLoaderPlugin;

impl Plugin for SwModuleLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<JsAssetHandles>()
            .add_systems(
                Update,
                receiver_request.run_if(resource_exists::<SwModuleLoaderRequestReceiver>),
            )
            .add_systems(
                Update,
                check_js_asset_ready.run_if(resource_exists::<SwModuleLoaderResponeSender>),
            );
    }
}

#[derive(Resource, Default)]
pub struct JsAssetHandles(pub Vec<Handle<JsAsset>>);

pub(super) fn receiver_request(
    asset_server: Res<AssetServer>,
    mut event_receiver: ResMut<SwModuleLoaderRequestReceiver>,
    mut js_asset_handles: ResMut<JsAssetHandles>,
    js_assets: Res<Assets<JsAsset>>,
    sender: Res<SwModuleLoaderResponeSender>,
) -> Result {
    info!("sss");
    if let Ok(SwModuleLoaderRequestEvent::LoadJsAsset(path)) = event_receiver.0.try_recv() {
        info!("try load module:{}", path);
        let asset = asset_server.load(path);
        if asset_server.is_loaded_with_dependencies(asset.id()) {
            sender
                .0
                .send(SwModuleLoaderResponeEvent::LoadedJsAsset(
                    js_assets.get(asset.id()).unwrap().clone(),
                ))
                .unwrap();
        } else {
            js_asset_handles.0.push(asset);
        }
    }

    Ok(())
}

fn check_js_asset_ready(
    asset_server: Res<AssetServer>,
    js_asset_handles: Res<JsAssetHandles>,
    js_assets: Res<Assets<JsAsset>>,
    mut events: EventReader<AssetEvent<JsAsset>>,
    sender: Res<SwModuleLoaderResponeSender>,
) -> Result {
    for event in events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } => {
                if js_asset_handles
                    .0
                    .contains(&asset_server.get_id_handle(*id).unwrap())
                {
                    info!("加载完成");
                    sender.0.send(SwModuleLoaderResponeEvent::LoadedJsAsset(
                        js_assets.get(*id).unwrap().clone(),
                    ))?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[derive(Default)]
pub struct SwModuleJobQueue {
    futures: Rc<RefCell<FuturesUnordered<FutureJob>>>,
    jobs: Rc<RefCell<VecDeque<NativeJob>>>,
}

impl Debug for SwModuleJobQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SimpleQueue").field(&"..").finish()
    }
}

impl SwModuleJobQueue {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl JobQueue for SwModuleJobQueue {
    fn enqueue_promise_job(&self, job: NativeJob, _: &mut Context) {
        self.jobs.borrow_mut().push_back(job);
    }

    fn enqueue_future_job(&self, future: FutureJob, _context: &mut Context) {
        self.futures.borrow().push(future);
    }

    fn run_jobs_async<'a, 'ctx, 'fut>(
        &'a self,
        context: &'ctx mut Context,
    ) -> Pin<Box<dyn Future<Output = ()> + 'fut>>
    where
        'a: 'fut,
        'ctx: 'fut,
    {
        Box::pin(async {
            if self.jobs.borrow().is_empty() && self.futures.borrow().is_empty() {
                return;
            }

            let context = RefCell::new(context);
            // Used to sync the finalization of both tasks
            let finished = Cell::new(0b00u8);
            let fut_queue = async {
                loop {
                    if self.futures.borrow().is_empty() {
                        finished.set(finished.get() | 0b01);
                        if finished.get() >= 0b11 {
                            // All possible futures and jobs were completed. Exit.
                            return;
                        }
                        // All possible jobs were completed, but `jqueue` could have
                        // pending jobs. Yield to the executor to try to progress on
                        // `jqueue` until we have more pending futures.
                        future::yield_now().await;
                        continue;
                    }
                    finished.set(finished.get() & 0b10);

                    // Blocks on all the enqueued futures, driving them all to completion.
                    let futures = &mut std::mem::take(&mut *self.futures.borrow_mut());

                    for fut in futures {
                        let job = fut.await;
                        self.enqueue_promise_job(job, &mut context.borrow_mut());
                        future::yield_now().await;
                    }
                    //while let Some(job) = futures.next().await {
                    //    // Important to schedule the returned `job` into the job queue, since that's
                    //    // what allows updating the `Promise` seen by ECMAScript for when the future
                    //    // completes.
                    //    self.enqueue_promise_job(job, &mut context.borrow_mut());
                    //}
                }
            };
            let job_queue = async {
                loop {
                    if self.jobs.borrow().is_empty() {
                        finished.set(finished.get() | 0b10);
                        if finished.get() >= 0b11 {
                            // All possible futures and jobs were completed. Exit.
                            return;
                        }
                        // All possible jobs were completed, but `fqueue` could have
                        // pending futures. Yield to the executor to try to progress on
                        // `fqueue` until we have more pending jobs.
                        future::yield_now().await;
                        continue;
                    };
                    finished.set(finished.get() & 0b01);

                    let jobs = std::mem::take(&mut *self.jobs.borrow_mut());
                    for job in jobs {
                        if let Err(e) = job.call(&mut context.borrow_mut()) {
                            eprintln!("Uncaught {e}");
                        }
                        future::yield_now().await;
                    }
                }
            };
            future::zip(fut_queue, job_queue).await;
        })
    }

    fn run_jobs(&self, _context: &mut Context) {
        panic!("SwModuleJobQueue needs use run_jobs_async");
    }
}
