use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
};

use bevy::{platform::collections::HashMap, prelude::*};
use boa_engine::{js_string, prelude::*, property::Attribute};
use boa_runtime::Console;

use crate::{
    custom::CustomTypedId,
    js_engine::{
        event::{JsEngineRequestEvent, JsEngineResponseEvent},
        host_defined::*,
        loader::SimpleWarfareModuleLoader,
        module::ModModule,
        simple_warfare_cli::{SimpleWarfareCli, SwRequestEvent, SwResponseEvent},
    },
};

pub struct JsEngine {
    pub(super) context: Context,
    pub(super) module_map: HashMap<String, Vec<ModModule>>,
    pub(super) custom_typed_id_generator: CustomTypedId,
}

impl JsEngine {
    pub fn new(
        loader: SimpleWarfareModuleLoader,
        js_engine_request_sender: Arc<Sender<JsEngineRequestEvent>>,
        js_engine_response_sender: Arc<Sender<JsEngineResponseEvent>>,
        sw_request_sender: Arc<Sender<SwRequestEvent>>,
        sw_response_receiver: Arc<Mutex<Receiver<SwResponseEvent>>>,
    ) -> Self {
        let context_builder = Context::builder();
        let loader = Rc::new(loader);

        let mut context = context_builder
            .module_loader(loader.clone())
            .build()
            .expect("Build Js Context error!");

        let ctx = RefCell::new(&mut context);

        insert_host_defined_data(&mut ctx.borrow_mut());
        egister_global_property(
            &mut ctx.borrow_mut(),
            js_engine_request_sender,
            js_engine_response_sender,
            sw_request_sender,
            sw_response_receiver,
        );

        register_global_callable(&mut ctx.borrow_mut());

        Self {
            context,
            module_map: HashMap::new(),
            custom_typed_id_generator: 0,
        }
    }
}

fn egister_global_property(
    ctx: &mut Context,
    js_engine_request_sender: Arc<Sender<JsEngineRequestEvent>>,
    js_engine_response_sender: Arc<Sender<JsEngineResponseEvent>>,
    sw_request_sender: Arc<Sender<SwRequestEvent>>,
    sw_response_receiver: Arc<Mutex<Receiver<SwResponseEvent>>>,
) {
    let console = Console::init(ctx);
    ctx.register_global_property(Console::NAME, console, Attribute::all())
        .expect("the console builtin shouldn't exist");

    let simple_warfare_cli = SimpleWarfareCli::init(
        ctx,
        js_engine_request_sender,
        js_engine_response_sender,
        sw_request_sender,
        sw_response_receiver,
    );

    ctx.register_global_property(SimpleWarfareCli::NAME, simple_warfare_cli, Attribute::all())
        .expect("the sw builtin shouldn't exist");
}

fn register_global_callable(ctx: &mut Context) {
    let moduleobj = JsObject::default();
    moduleobj
        .set(
            js_string!("exports"),
            JsValue::from(js_string!(" ")),
            false,
            ctx,
        )
        .unwrap();

    ctx.register_global_property(
        js_string!("module"),
        JsValue::from(moduleobj),
        Attribute::default(),
    )
    .unwrap();
}

fn insert_host_defined_data(ctx: &mut Context) {
    ctx.realm().host_defined_mut().insert(UnitMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(SelectedSignalMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(OnUnitEnterSignalMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(OnUnitExitSignalMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(NewWayPointSignalMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(SignalEntityMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(JsObjectMap::default());
    ctx.realm().host_defined_mut().insert(JsProxyMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(CustomInnerInfoMap::default());
    ctx.realm()
        .host_defined_mut()
        .insert(ModulePathToCustomTypedIdMap::default());
}
