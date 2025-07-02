mod context;
mod engine;
pub mod event;
pub mod loader;
pub mod module;
pub mod plugin;
use std::sync::{
    Arc, Mutex,
    mpsc::{self, Receiver, Sender},
};

use crate::{
    app_state::AppState,
    js_engine::{
        context::*,
        engine::{JsEngine, SwRequireLoaderRequestReceiver, SwRequireLoaderResponeSender},
        event::{JsEngineRequestEvent, JsEngineResponeEvent},
        loader::{
            SimpleWarfareModuleLoader, SwModuleLoaderRequestReceiver, SwModuleLoaderResponeSender,
        },
        plugin::SwLoaderPlugin,
    },
};
use bevy::prelude::*;
use boa_engine::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsEngineError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("")]
    BoaEngine(#[from] JsError),
}

pub struct JsEnginePlugin;

impl Plugin for JsEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SwLoaderPlugin)
            .add_event::<JsEngineRequestEvent>()
            .add_systems(OnEnter(AppState::InitJsContext), init_js_context)
            .add_systems(
                Update,
                engine_inited.run_if(
                    in_state(AppState::InitJsContext)
                        .and(resource_exists::<JsEngineEventResponeReciver>),
                ),
            );
    }
}

#[derive(Resource)]
pub struct JsEngineEventRequestSender(pub Arc<Sender<JsEngineRequestEvent>>);
#[derive(Resource)]
pub struct JsEngineEventResponeReciver(pub Arc<Mutex<Receiver<JsEngineResponeEvent>>>);

fn init_js_context(mut commands: Commands) -> Result {
    //与js线程的双向通道
    let (je_request_sender, je_request_receiver) = mpsc::channel();
    let (je_respone_sender, je_respone_receiver) = mpsc::channel();

    commands.insert_resource(JsEngineEventRequestSender(Arc::new(je_request_sender)));
    commands.insert_resource(JsEngineEventResponeReciver(Arc::new(Mutex::new(
        je_respone_receiver,
    ))));

    let (sw_module_request_sender, sw_module_request_receiver) = mpsc::channel();
    let (sw_module_respone_sender, sw_module_respone_receiver) = mpsc::channel();
    commands.insert_resource(SwModuleLoaderResponeSender(Arc::new(
        sw_module_respone_sender.clone(),
    )));
    commands.insert_resource(SwModuleLoaderRequestReceiver(Arc::new(Mutex::new(
        sw_module_request_receiver,
    ))));

    let (sw_require_request_sender, sw_require_request_receiver) = mpsc::channel();
    let (sw_require_respone_sender, sw_require_respone_receiver) = mpsc::channel();
    commands.insert_resource(SwRequireLoaderResponeSender(Arc::new(
        sw_require_respone_sender.clone(),
    )));
    commands.insert_resource(SwRequireLoaderRequestReceiver(Arc::new(Mutex::new(
        sw_require_request_receiver,
    ))));

    std::thread::spawn(move || {
        let engine = &mut JsEngine::new(
            SimpleWarfareModuleLoader::new(
                Arc::new(sw_module_request_sender),
                Arc::new(Mutex::new(sw_module_respone_receiver)),
            )
            .unwrap(),
            Arc::new(sw_require_request_sender),
            Arc::new(Mutex::new(sw_require_respone_receiver))
        );
        let je_respone_sender = Arc::new(je_respone_sender);
        // 开始监听
        // 由bevy的EventWriter写入事件并经js_event_bridge中转到此
        je_respone_sender
            .send(JsEngineResponeEvent::EngineInited)
            .expect("Faied to send EngineInited event");

        while let Ok(event) = je_request_receiver.recv() {
            process_js_event(engine, event, je_respone_sender.clone())
                .expect("process_js_event error")
        }
    });
    Ok(())
}

fn engine_inited(
    mut next_state: ResMut<NextState<AppState>>,
    event_receiver: Res<JsEngineEventResponeReciver>,
) -> Result<()> {
    if let Ok(JsEngineResponeEvent::EngineInited) = event_receiver
        .0
        .lock()
        .expect("lock js respone receiver error in the system `engine_inited`")
        .try_recv()
    {
        next_state.set(AppState::ModInfoLoading);
    }

    Ok(())
}
