mod context;
mod engine;
pub mod event;
pub mod module;

use std::{
    rc::Rc,
    sync::{Mutex, mpsc},
};

use crate::{
    app_state::AppState,
    assets::mods::js::JsAsset,
    js_engine::{context::*, engine::JsEngine, event::JsEngineEvent},
};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, block_on, futures_lite::future},
};
use boa_engine::prelude::*;
use std::sync::mpsc::{Receiver, Sender};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmilodonEngineError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("")]
    BoaEngine(#[from] JsError),
}

pub struct SmilodonEnginePlugin;

impl Plugin for SmilodonEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JsEngineEvent>()
            .add_systems(PreStartup, load_libs)
            .add_systems(Update, check_libs.run_if(in_state(AppState::LibsLoading)))
            .add_systems(OnEnter(AppState::LibsLoaded), init_js_context)
            .add_systems(
                Update,
                handle_task.run_if(resource_exists::<ComputeJsContext>),
            )
            .add_systems(Update, smilodon_event_bridge)
            .add_systems(Update, engine_inited.run_if(in_state(AppState::LibsLoaded)));
    }
}

#[derive(Resource)]
pub struct SimpleWarfareEngineHandle(pub Handle<JsAsset>);

#[derive(Resource)]
struct JsEngineEventSender(Sender<JsEngineEvent>);
#[derive(Resource)]
struct JsEngineEventReciver(Mutex<Receiver<JsEngineEvent>>);

fn load_libs(mut commands: Commands, asset_server: Res<AssetServer>) {
    //加载基本的Js Modules
    commands.insert_resource(SimpleWarfareEngineHandle(
        asset_server.load::<JsAsset>("mod_libs/simple_warfare_engine.js"),
    ));
}

fn check_libs(
    mut next_state: ResMut<NextState<AppState>>,
    simple_warfare_engine_file: Res<SimpleWarfareEngineHandle>,
    mut events: EventReader<AssetEvent<JsAsset>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&simple_warfare_engine_file.0) {
            //开始建立Js运行时
            next_state.set(AppState::LibsLoaded);
        }
    }
}

#[derive(Debug, Error)]
pub enum JsEngineError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] Box<dyn std::error::Error>),
}

#[derive(Resource)]
struct ComputeJsContext(Task<Result>);

fn init_js_context(
    mut commands: Commands,
    engine_handle: Res<SimpleWarfareEngineHandle>,
    js_assets: Res<Assets<JsAsset>>,
) -> Result {
    if let Some(engine_js) = js_assets.get(engine_handle.0.id()) {
        let engine_js_code = engine_js.context.clone();

        //与js线程的双向通道
        let (sender, rx) = mpsc::channel();
        let (tx, receiver) = mpsc::channel();

        commands.insert_resource(JsEngineEventSender(sender));
        commands.insert_resource(JsEngineEventReciver(Mutex::new(receiver)));
        let task = AsyncComputeTaskPool::get().spawn::<Result>(async move {
            let rx = rx;
            let tx = tx;
            let engine = &mut JsEngine::new(&engine_js_code);

            // 开始监听
            // 由bevy的EventWriter写入事件并经js_event_bridge中转到此
            tx.send(JsEngineEvent::EngineInited).expect("sd");
            while let Ok(event) = rx.recv() {
                process_js_event(engine, event, &tx).expect("process_js_event error")
            }
            Ok(())
        });
        commands.insert_resource(ComputeJsContext(task));
        //Js运行时单独在一个线程内运行
        Ok(())
    } else {
        Err(BevyError::from("the js libs didn't found"))
    }
}

fn handle_task(mut js_ontext_task: ResMut<ComputeJsContext>) -> Result {
    if let Some(Err(e)) = block_on(future::poll_once(&mut js_ontext_task.0)) {
        return Err(e);
    }
    Ok(())
}

fn smilodon_event_bridge(
    mut event_reader: EventReader<JsEngineEvent>,
    event_sender: Option<Res<JsEngineEventSender>>,
) -> Result<()> {
    if let Some(event_sender) = event_sender {
        for event in event_reader.read() {
            event_sender.0.send(event.clone())?
        }
    }

    Ok(())
}

fn engine_inited(
    mut next_state: ResMut<NextState<AppState>>,
    event_receiver: Option<Res<JsEngineEventReciver>>,
) -> Result<()> {
    if let Some(event_receiver) = event_receiver {
        if let Ok(event) = event_receiver.0.lock().expect("").recv() {
            if let JsEngineEvent::EngineInited = event {
                next_state.set(AppState::ModInfoLoading);
            }
        }
    }

    Ok(())
}
