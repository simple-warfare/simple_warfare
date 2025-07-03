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
        engine::JsEngine,
        event::{JsEngineRequestEvent, JsEngineResponseEvent},
        loader::{
            SimpleWarfareModuleLoader, SwModuleLoaderRequestReceiver, SwModuleLoaderResponseSender,
            SwRequireLoaderRequestReceiver, SwRequireLoaderResponseSender,
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
            .add_event::<JsEngineResponseEvent>()
            .add_systems(OnEnter(AppState::InitJsContext), init_js_context)
            .add_systems(
                Update,
                broadcast_js_engine_response_event
                    .run_if(resource_exists::<JsEngineEventResponseReciver>),
            )
            .add_systems(
                Update,
                inited_js_engine.run_if(
                    in_state(AppState::InitJsContext)
                        .and(resource_exists::<JsEngineEventResponseReciver>),
                ),
            );
    }
}

#[derive(Resource)]
pub struct JsEngineEventRequestSender(pub Arc<Sender<JsEngineRequestEvent>>);
#[derive(Resource)]
pub struct JsEngineEventResponseReciver(pub Arc<Mutex<Receiver<JsEngineResponseEvent>>>);

fn init_js_context(mut commands: Commands) -> Result {
    //与js线程的双向通道
    let (je_request_sender, je_request_receiver) = mpsc::channel();
    let (je_response_sender, je_response_receiver) = mpsc::channel();

    commands.insert_resource(JsEngineEventRequestSender(Arc::new(je_request_sender)));
    commands.insert_resource(JsEngineEventResponseReciver(Arc::new(Mutex::new(
        je_response_receiver,
    ))));

    let (sw_module_request_sender, sw_module_request_receiver) = mpsc::channel();
    let (sw_module_response_sender, sw_module_response_receiver) = mpsc::channel();
    commands.insert_resource(SwModuleLoaderResponseSender(Arc::new(
        sw_module_response_sender.clone(),
    )));
    commands.insert_resource(SwModuleLoaderRequestReceiver(Arc::new(Mutex::new(
        sw_module_request_receiver,
    ))));

    let (sw_require_request_sender, sw_require_request_receiver) = mpsc::channel();
    let (sw_require_response_sender, sw_require_response_receiver) = mpsc::channel();
    commands.insert_resource(SwRequireLoaderResponseSender(Arc::new(
        sw_require_response_sender.clone(),
    )));
    commands.insert_resource(SwRequireLoaderRequestReceiver(Arc::new(Mutex::new(
        sw_require_request_receiver,
    ))));

    std::thread::spawn(move || {
        let engine = &mut JsEngine::new(
            SimpleWarfareModuleLoader::new(
                Arc::new(sw_module_request_sender),
                Arc::new(Mutex::new(sw_module_response_receiver)),
            )
            .unwrap(),
            Arc::new(sw_require_request_sender),
            Arc::new(Mutex::new(sw_require_response_receiver)),
        );
        let je_response_sender = Arc::new(je_response_sender);
        // 开始监听
        // 由bevy的EventWriter写入事件并经js_event_bridge中转到此
        je_response_sender
            .send(JsEngineResponseEvent::EngineInited)
            .expect("Faied to send EngineInited event");

        while let Ok(event) = je_request_receiver.recv() {
            process_js_event(engine, event, je_response_sender.clone())
                .expect("process_js_event error")
        }
    });
    Ok(())
}

fn broadcast_js_engine_response_event(
    mut event_writer: EventWriter<JsEngineResponseEvent>,
    event_receiver: Res<JsEngineEventResponseReciver>,
) -> Result<()> {
    if let Ok(event) = event_receiver
        .0
        .lock()
        .expect("lock js Response receiver error in the system `engine_inited`")
        .try_recv()
    {
        event_writer.write(event);
    }

    Ok(())
}

fn inited_js_engine(
    mut next_state: ResMut<NextState<AppState>>,
    mut event_reader: EventReader<JsEngineResponseEvent>,
) {
    for event in event_reader.read() {
        match event {
            JsEngineResponseEvent::EngineInited => next_state.set(AppState::ModInfoLoading),
            _ => {}
        }
    }
}
