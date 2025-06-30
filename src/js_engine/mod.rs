mod context;
mod engine;
pub mod event;
pub mod module;

use std::sync::Arc;

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
        app.add_event::<JsEngineEvent>()
            .add_systems(PreStartup, load_libs)
            .add_systems(
                Update,
                check_libs.run_if(
                    in_state(AppState::LibsLoading)
                        .and(resource_exists::<SimpleWarfareEngineHandle>),
                ),
            )
            .add_systems(OnEnter(AppState::LibsLoaded), init_js_context)
            .add_systems(
                Update,
                handle_task.run_if(resource_exists::<ComputeJsContext>),
            )
            .add_systems(
                Update,
                engine_inited.run_if(
                    in_state(AppState::LibsLoaded).and(resource_exists::<JsEngineEventReciver>),
                ),
            );
    }
}

#[derive(Resource)]
pub struct SimpleWarfareEngineHandle(pub Handle<JsAsset>);

#[derive(Resource)]
pub struct JsEngineEventSender(pub Arc<Sender<JsEngineEvent>>);
#[derive(Resource)]
pub struct JsEngineEventReciver(pub Receiver<JsEngineEvent>);

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
        info!("LibsLoading");
        if event.is_loaded_with_dependencies(&simple_warfare_engine_file.0) {
            //开始建立Js运行时
            info!("LibsLoaded");
            next_state.set(AppState::LibsLoaded);
        }
    }
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
        let (sender, rx) = mpsc::unbounded_channel();
        let (tx, receiver) = mpsc::unbounded_channel();

        commands.insert_resource(JsEngineEventSender(Arc::new(sender)));
        commands.insert_resource(JsEngineEventReciver(receiver));
        let task = AsyncComputeTaskPool::get().spawn_local::<Result>(async move {
            let mut rx = rx;
            let tx = tx;

            let engine = &mut JsEngine::new(&engine_js_code);

            // 开始监听
            // 由bevy的EventWriter写入事件并经js_event_bridge中转到此
            tx.send(JsEngineEvent::EngineInited)
                .expect("Faied to send EngineInited event");

            while let Some(event) = rx.recv().await {
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

fn engine_inited(
    mut next_state: ResMut<NextState<AppState>>,
    mut event_receiver: ResMut<JsEngineEventReciver>,
) -> Result<()> {
    if let Ok(JsEngineEvent::EngineInited) = event_receiver.0.try_recv() {
        next_state.set(AppState::ModInfoLoading);
    }

    Ok(())
}
