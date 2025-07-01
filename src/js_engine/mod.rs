mod context;
mod engine;
pub mod event;
pub mod loader;
pub mod module;
use std::sync::Arc;

use crate::{
    app_state::AppState,
    js_engine::{
        context::*,
        engine::JsEngine,
        event::{JsEngineRequestEvent, JsEngineResponeEvent},
        loader::{
            SimpleWarfareModuleLoader, SwModuleLoaderPlugin, SwModuleLoaderRequestReceiver,
            SwModuleLoaderResponeSender,
        },
    },
};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, block_on, futures_lite::future},
};
use boa_engine::prelude::*;
use thiserror::Error;
use tokio::sync::mpsc::{self, UnboundedReceiver as Receiver, UnboundedSender as Sender};

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
        app.add_plugins(SwModuleLoaderPlugin)
            .add_event::<JsEngineRequestEvent>()
            .add_systems(OnEnter(AppState::InitJsContext), init_js_context)
            .add_systems(
                Update,
                handle_task.run_if(resource_exists::<ComputeJsContext>),
            )
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
pub struct JsEngineEventResponeReciver(pub Receiver<JsEngineResponeEvent>);

#[derive(Resource)]
struct ComputeJsContext(Task<Result>);

fn init_js_context(mut commands: Commands) -> Result {
    //与js线程的双向通道
    let (je_request_sender, mut je_request_receiver) = mpsc::unbounded_channel();
    let (je_respone_sender, je_respone_receiver) = mpsc::unbounded_channel();

    commands.insert_resource(JsEngineEventRequestSender(Arc::new(je_request_sender)));
    commands.insert_resource(JsEngineEventResponeReciver(je_respone_receiver));

    let (sw_module_request_sender, sw_module_request_receiver) = mpsc::unbounded_channel();
    let (sw_module_respone_sender, sw_module_respone_receiver) = mpsc::unbounded_channel();
    commands.insert_resource(SwModuleLoaderResponeSender(Arc::new(
        sw_module_respone_sender,
    )));
    commands.insert_resource(SwModuleLoaderRequestReceiver(sw_module_request_receiver));

    let task = AsyncComputeTaskPool::get().spawn_local::<Result>(async move {
        let engine = &mut JsEngine::new(
            SimpleWarfareModuleLoader::new(sw_module_request_sender, sw_module_respone_receiver)
                .unwrap(),
        );

        // 开始监听
        // 由bevy的EventWriter写入事件并经js_event_bridge中转到此
        je_respone_sender
            .send(JsEngineResponeEvent::EngineInited)
            .expect("Faied to send EngineInited event");

        while let Some(event) = je_request_receiver.recv().await {
            process_js_event(engine, event, &je_respone_sender)
                .await
                .expect("process_js_event error")
        }
        Ok(())
    }).detach();
    //commands.insert_resource(ComputeJsContext(task));
    //Js运行时单独在一个线程内运行
    Ok(())
}

fn handle_task(mut js_ontext_task: ResMut<ComputeJsContext>) -> Result {
    if let Some(Err(e)) = block_on(future::poll_once(&mut js_ontext_task.0)) {
        return Err(e);
    }

    Ok(())
}

fn engine_inited(
    mut next_state: ResMut<NextState<AppState>>,
    mut event_receiver: ResMut<JsEngineEventResponeReciver>,
) -> Result<()> {
    if let Ok(JsEngineResponeEvent::EngineInited) = event_receiver.0.try_recv() {
        next_state.set(AppState::ModInfoLoading);
    }

    Ok(())
}
