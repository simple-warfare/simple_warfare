mod context;
pub mod event;
use std::rc::Rc;

use crate::{
    app_state::AppState,
    assets::mods::js::JsAsset,
    js_engine::{context::*, event::JsEngineEvent},
};
use bevy::{
    platform::collections::HashMap,
    prelude::*,
    tasks::{AsyncComputeTaskPool, IoTaskPool},
};
use boa_engine::{module::SimpleModuleLoader, prelude::*};
use crossbeam_channel::{Receiver, Sender, select};
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
            .add_systems(Update, check_libs.run_if(in_state(AppState::LibsLoding)))
            .add_systems(OnEnter(AppState::LibsLoded), init_js_context)
            .add_systems(Update, smilodon_event_bridge)
            .add_systems(Update, engine_inited.run_if(in_state(AppState::LibsLoded)));
    }
}

#[derive(Resource)]
pub struct SimpleWarfareEngineHandle(pub Handle<JsAsset>);

#[derive(Resource)]
struct JsEngineEventSender(Sender<JsEngineEvent>);
#[derive(Resource)]
struct JsEngineEventReciver(Receiver<JsEngineEvent>);

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
            next_state.set(AppState::LibsLoded);
        }
    }
}

#[derive(Debug, Error)]
pub enum JsEngineError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] Box<dyn std::error::Error>),
}

fn init_js_context(
    mut commands: Commands,
    engine_handle: Res<SimpleWarfareEngineHandle>,
    js_assets: Res<Assets<JsAsset>>,
) -> Result {
    if let Some(engine_js) = js_assets.get(engine_handle.0.id()) {
        let engine_js_code = engine_js.context.clone();

        //与js线程的双向通道
        let (sender, rx) = crossbeam_channel::bounded(100);
        let (tx, receiver) = crossbeam_channel::bounded(100);

        commands.insert_resource(JsEngineEventSender(sender));
        commands.insert_resource(JsEngineEventReciver(receiver));
        std::thread::spawn(move || {
            let rx = rx;
            let tx = tx;
            let module_map = &mut HashMap::new();

            let context_builder = Context::builder();
            let loader =
                Rc::new(SimpleModuleLoader::new("./assets/mod_libs").expect("load mod_libs error"));

            let context = &mut context_builder
                .module_loader(loader.clone())
                .build()
                .expect("Build Js Context error!");
            add_runtime(context);
            register_class(context);

            let module =
                load_mod_libs(context, loader.clone(), engine_js_code).expect("load module error");
            module_map.insert("simple_warfare_engine", module);
            // 开始监听
            // 由bevy的EventWriter写入事件并经js_event_bridge中转到此
            tx.send(JsEngineEvent::EngineInited).expect("sd");
            while let Ok(event) = rx.recv() {
                process_js_event(context, module_map, event).expect("process_js_event error")
            }
        });
        //Js运行时单独在一个线程内运行
        Ok(())
    } else {
        Err(BevyError::from("the js libs didn't found"))
    }
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
        if let Ok(event) = event_receiver.0.recv() {
            if let JsEngineEvent::EngineInited = event {
                next_state.set(AppState::ModInfoLoading);
            }
        }
    }

    Ok(())
}
