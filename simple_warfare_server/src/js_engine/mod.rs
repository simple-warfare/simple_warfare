mod context;
mod engine;
pub mod event;
pub mod global;
pub mod host_defined;
pub mod loader;
pub mod module;
pub mod plugin;
pub mod signal;
pub mod simple_warfare_cli;
pub mod synchronize;

use std::sync::{
    Arc, Mutex,
    mpsc::{self, Receiver, Sender},
};

use crate::{
    js_engine::{
        context::*,
        engine::JsEngine,
        event::{EventPlugin, JsEngineRequestEvent, JsEngineResponseEvent},
        loader::{SimpleWarfareModuleLoader, SwModuleLoaderRequestReceiver},
        plugin::SwLoaderPlugin,
        simple_warfare_cli::{SwCliRequestReceiver, SwCliResponseSender, plugin::SwPlugin},
    },
    statistics::ServerState,
};
use bevy::prelude::*;
use boa_engine::prelude::*;
use thiserror::Error;

use self::simple_warfare_cli::{
    io::fs::{SwFsRequestReceiver, SwFsResponseSender},
};

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
            .add_plugins(EventPlugin)
            .add_plugins(SwPlugin)
            .add_systems(OnEnter(ServerState::JsContextInitiating), init_js_context)
            .add_systems(
                Update,
                broadcast_js_engine_response_event
                    .run_if(resource_exists::<JsEngineResponseReciver>),
            )
            .add_systems(
                Update,
                inited_js_engine.run_if(
                    in_state(ServerState::JsContextInitiating)
                        .and(resource_exists::<JsEngineResponseReciver>),
                ),
            );
    }
}

#[derive(Resource)]
pub struct JsEngineRequestSender(pub Arc<Sender<JsEngineRequestEvent>>);
#[derive(Resource)]
pub struct JsEngineResponseReciver(pub Arc<Mutex<Receiver<JsEngineResponseEvent>>>);

fn init_js_context(mut commands: Commands) -> Result {
    //与js线程的双向通道
    let (js_request_sender, js_request_receiver) = mpsc::channel();
    let (js_response_sender, js_response_receiver) = mpsc::channel();
    let js_request_sender = Arc::new(js_request_sender);
    commands.insert_resource(JsEngineRequestSender(js_request_sender.clone()));
    commands.insert_resource(JsEngineResponseReciver(Arc::new(Mutex::new(
        js_response_receiver,
    ))));

    let (sw_module_request_sender, sw_module_request_receiver) = mpsc::channel();

    commands.insert_resource(SwModuleLoaderRequestReceiver(Arc::new(Mutex::new(
        sw_module_request_receiver,
    ))));

    let (sw_cli_request_sender, sw_cli_request_receiver) = mpsc::channel();
    let (sw_cli_response_sender, sw_cli_response_receiver) = mpsc::channel();
    commands.insert_resource(SwCliResponseSender(Arc::new(
        sw_cli_response_sender.clone(),
    )));
    commands.insert_resource(SwCliRequestReceiver(Arc::new(Mutex::new(
        sw_cli_request_receiver,
    ))));

    let (sw_fs_request_sender, sw_fs_request_receiver) = mpsc::channel();
    let (sw_fs_response_sender, sw_fs_response_receiver) = mpsc::channel();
    commands.insert_resource(SwFsResponseSender(Arc::new(sw_fs_response_sender.clone())));
    commands.insert_resource(SwFsRequestReceiver(Arc::new(Mutex::new(
        sw_fs_request_receiver,
    ))));

    let js_response_sender = Arc::new(js_response_sender.clone());

    std::thread::spawn(move || {
        let engine = &mut JsEngine::new(
            SimpleWarfareModuleLoader::new("assets/", Arc::new(sw_module_request_sender)).unwrap(),
            js_request_sender.clone(),
            js_response_sender.clone(),
            Arc::new(sw_cli_request_sender),
            Arc::new(Mutex::new(sw_cli_response_receiver)),
            Arc::new(sw_fs_request_sender),
        );
        // 开始监听
        // 由bevy的EventWriter写入事件并经js_event_bridge中转到此
        js_response_sender
            .send(JsEngineResponseEvent::EngineInited)
            .expect("Faied to send EngineInited event");

        while let Ok(event) = js_request_receiver.recv() {
            process_js_event(
                engine,
                event,
                js_request_sender.clone(),
                js_response_sender.clone(),
            )
            .expect("process_js_event error")
        }
    });
    Ok(())
}

fn broadcast_js_engine_response_event(
    mut event_writer: EventWriter<JsEngineResponseEvent>,
    event_receiver: Res<JsEngineResponseReciver>,
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
    mut next_state: ResMut<NextState<ServerState>>,
    mut event_reader: EventReader<JsEngineResponseEvent>,
) {
    for event in event_reader.read() {
        if let JsEngineResponseEvent::EngineInited = event {
            info!("ModSet Loading");
            next_state.set(ServerState::ModSetLoading)
        }
    }
}
