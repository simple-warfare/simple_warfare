pub mod assets;
pub mod event;
use std::{path::Path, rc::Rc};

use crate::{
    app_state::AppState,
    smilodon_engine::{
        assets::{AssetsPlugin, js::JsAsset},
        event::SmilodonEngineEvent,
    },
    unit::section::core::Core,
};
use bevy::{prelude::*, tasks::IoTaskPool};
use boa_engine::{
    builtins::promise::PromiseState, module::SimpleModuleLoader, prelude::*, property::Attribute,
};
use boa_runtime::Console;
use crossbeam_channel::{Receiver, Sender};
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
        app.add_event::<SmilodonEngineEvent>()
            .add_plugins(AssetsPlugin)
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
struct SmilodonEngineEventSender(Sender<SmilodonEngineEvent>);
#[derive(Resource)]
struct SmilodonEngineEventReciver(Receiver<SmilodonEngineEvent>);

fn load_libs(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            next_state.set(AppState::LibsLoded);
        }
    }
}

fn init_js_context(
    mut commands: Commands,
    simple_warfare_engine_handle: Res<SimpleWarfareEngineHandle>,
    js_assets: Res<Assets<JsAsset>>,
) {
    let simple_warfare_engine_js = js_assets
        .get(simple_warfare_engine_handle.0.id())
        .unwrap()
        .context
        .clone();
    let (sender, rx) = crossbeam_channel::bounded(100);
    let (tx, receiver) = crossbeam_channel::bounded(100);
    commands.insert_resource(SmilodonEngineEventSender(sender));
    commands.insert_resource(SmilodonEngineEventReciver(receiver));
    let _task = IoTaskPool::get()
        .spawn_local::<Result<(), Box<dyn std::error::Error>>>(async move {
            let rx = rx;
            let tx = tx;

            let context_builder = Context::builder();
            let loader = Rc::new(SimpleModuleLoader::new("./assets/mod_libs")?);

            let context = &mut context_builder
                .module_loader(loader.clone())
                .build()
                .expect("Build Js Context error!");
            add_runtime(context);
            register_class(context);

            let module = load_mod_libs(context, loader.clone(), simple_warfare_engine_js)?;

            let namespace = module.namespace(context);
            let code_js = r#"
            import { Core } from "./simple_warfare_engine.mjs"
            console.log("new Core")
            let core = new Core("坦克", 1000, 1000)
            console.log(JSON.stringify(core))
            "#;

            let module = Module::parse(
                Source::from_reader(code_js.as_bytes(), Some(Path::new("./tank.mjs"))),
                Some(module.realm().clone()),
                context,
            )
            .unwrap();
            let promise = module.load_link_evaluate(context);
            tx.send(SmilodonEngineEvent::EngineInited)?;
            context.run_jobs();

            assert_eq!(
                promise.state(),
                PromiseState::Fulfilled(JsValue::undefined())
            );
            Ok(())
        })
        .detach();
}

fn load_mod_libs(
    context: &mut Context,
    loader: Rc<SimpleModuleLoader>,
    simple_warfare_engine_js: String,
) -> Result<Module, Box<dyn std::error::Error>> {
    let source = Source::from_reader(
        simple_warfare_engine_js.as_bytes(),
        Some(Path::new("./simple_warfare_engine.mjs")),
    );

    let module = Module::parse(source, None, context).unwrap();

    loader.insert(
        Path::new("./assets/mod_libs")
            .canonicalize()?
            .join("simple_warfare_engine.mjs"),
        module.clone(),
    );

    let promise_result = module
        .load(context)
        .then(
            Some(
                NativeFunction::from_copy_closure_with_captures(
                    |_, _, module, context| {
                        module.link(context).unwrap();
                        Ok(JsValue::undefined())
                    },
                    module.clone(),
                )
                .to_js_function(context.realm()),
            ),
            None,
            context,
        )
        .then(
            Some(
                NativeFunction::from_copy_closure_with_captures(
                    |_, _, module, context| Ok(module.evaluate(context).into()),
                    module.clone(),
                )
                .to_js_function(context.realm()),
            ),
            None,
            context,
        );

    context.run_jobs();

    match promise_result.state() {
        PromiseState::Pending => {
            return Err("module didn't execute!".into());
        }
        PromiseState::Fulfilled(v) => {
            assert_eq!(v, JsValue::undefined());
        }
        PromiseState::Rejected(err) => {
            return Err(JsError::from_opaque(err)
                .try_native(context)
                .unwrap()
                .into());
        }
    }
    Ok(module)
}

fn smilodon_event_bridge(
    mut event_reader: EventReader<SmilodonEngineEvent>,
    event_sender: Option<Res<SmilodonEngineEventSender>>,
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
    event_receiver: Option<Res<SmilodonEngineEventReciver>>,
) -> Result<()> {
    if let Some(event_receiver) = event_receiver {
        if let Ok(event) = event_receiver.0.recv() {
            if let SmilodonEngineEvent::EngineInited = event {
                next_state.set(AppState::ModInfoLoading);
            }
        }
    }

    Ok(())
}
fn add_runtime(context: &mut Context) {
    let console = Console::init(context);
    context
        .register_global_property(Console::NAME, console, Attribute::all())
        .expect("the console builtin shouldn't exist");
}

fn register_class(context: &mut Context) {
    context
        .register_global_class::<Core>()
        .expect("the Core builtin shouldn't exist");
}
