pub mod io;
pub mod plugin;
pub mod server;

use bevy::prelude::*;
use boa_engine::{
    JsArgs, JsResult, js_string,
    object::{
        ObjectInitializer,
        builtins::{JsArray, JsProxy},
    },
    prelude::*,
    property::Attribute,
    value::{TryFromJs, TryIntoJs},
};
use std::{
    path::PathBuf,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
};

use crate::{
    assets::js_file::{section::SectionFile, toml::TomlFile},
    bevy_ext::try_from_js::vec2_try_from_js,
    custom::unit::section::{core::Core, movement::Movement},
    js_engine::{
        context::emit_signal,
        global::class::entity::JsEntity,
        host_defined::*,
        signal::JsSignalType,
        synchronize::{SynchronizeData, SynchronizeDataType},
    },
};

use self::{
    io::fs::{Fs, SwFsRequestEvent},
    server::init_server_objects,
};

use super::event::{JsEngineRequestEvent, JsEngineResponseEvent};

#[derive(Resource)]
pub struct SwCliRequestReceiver(pub Arc<Mutex<Receiver<SwCliRequestEvent>>>);

#[derive(Resource, Clone)]
pub struct SwCliResponseSender(pub Arc<Sender<SwCliResponseEvent>>);

#[derive(Debug, Default, Trace, Finalize, JsData)]
pub struct SimpleWarfareCli;

#[derive(Event)]
pub enum SwCliRequestEvent {
    RegisterEntity,
}

#[derive(Event, Clone)]
pub enum SwCliResponseEvent {
    None,
    RegisteredEntity(Entity),
}

#[derive(Debug, Clone)]
pub enum TeleportType {
    Position { this: Entity, position: Vec2 },
    Entity { this: Entity, target: Entity },
}

#[derive(Debug, Clone)]
pub enum LookType {
    Position { this: Entity, position: Vec2 },
    Entity { this: Entity, target: Entity },
}

#[derive(Debug, Clone, Copy)]
pub enum JsTargetType {
    Position,
    Entity,
}

impl TeleportType {
    pub fn entity(this: Entity, target: Entity) -> Self {
        Self::Entity { this, target }
    }
    pub fn position(this: Entity, position: Vec2) -> Self {
        Self::Position { this, position }
    }
}

impl LookType {
    pub fn entity(this: Entity, target: Entity) -> Self {
        Self::Entity { this, target }
    }
    pub fn position(this: Entity, position: Vec2) -> Self {
        Self::Position { this, position }
    }
}

impl TryFromJs for JsTargetType {
    fn try_from_js(value: &JsValue, _context: &mut Context) -> JsResult<Self> {
        match value {
            JsValue::String(target_type) => match target_type.to_std_string_lossy().as_str() {
                "Position" => Ok(JsTargetType::Position),
                "Entity" => Ok(JsTargetType::Entity),
                _ => Err(JsNativeError::typ()
                    .with_message("cannot convert value to a JsTeleportType")
                    .into()),
            },
            _ => Err(JsNativeError::typ()
                .with_message("cannot convert value to a JsTeleportType")
                .into()),
        }
    }
}

/// 创建Sw这个Js端的全局对象
impl SimpleWarfareCli {
    pub const NAME: JsString = js_string!("cli");

    pub fn init(
        context: &mut Context,
        js_engine_request_sender: Arc<Sender<JsEngineRequestEvent>>,
        js_engine_response_sender: Arc<Sender<JsEngineResponseEvent>>,
        sw_cli_request_sender: Arc<Sender<SwCliRequestEvent>>,
        sw_cli_response_receiver: Arc<Mutex<Receiver<SwCliResponseEvent>>>,
        sw_fs_request_sender: Arc<Sender<SwFsRequestEvent>>,
    ) -> JsObject {
        // Js传送单位位置的方法
        let teleport = unsafe {
            let js_engine_request_sender = js_engine_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let js_teleport_type = JsTargetType::try_from_js(args.first().unwrap(), ctx)?;
                match js_teleport_type {
                    JsTargetType::Position => js_engine_request_sender
                        .send(JsEngineRequestEvent::ToTeleport(TeleportType::position(
                            JsEntity::try_from_js(args.get(1).unwrap(), ctx)?.to_entity(),
                            vec2_try_from_js(args.get(2).unwrap(), ctx)?,
                        )))
                        .unwrap(),

                    JsTargetType::Entity => js_engine_request_sender
                        .send(JsEngineRequestEvent::ToLook(LookType::entity(
                            JsEntity::try_from_js(args.get(1).unwrap(), ctx)?.to_entity(),
                            JsEntity::try_from_js(args.get(2).unwrap(), ctx)?.to_entity(),
                        )))
                        .unwrap(),
                }
                Ok(JsValue::undefined())
            })
        };

        // Js设置单位朝向的方法
        let look_at = unsafe {
            let js_engine_request_sender = js_engine_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let js_look_type = JsTargetType::try_from_js(args.first().unwrap(), ctx)?;
                match js_look_type {
                    JsTargetType::Position => js_engine_request_sender
                        .send(JsEngineRequestEvent::ToLook(LookType::position(
                            JsEntity::try_from_js(args.get(1).unwrap(), ctx)?.to_entity(),
                            vec2_try_from_js(args.get(2).unwrap(), ctx)?,
                        )))
                        .unwrap(),

                    JsTargetType::Entity => js_engine_request_sender
                        .send(JsEngineRequestEvent::ToLook(LookType::entity(
                            JsEntity::try_from_js(args.get(1).unwrap(), ctx)?.to_entity(),
                            JsEntity::try_from_js(args.get(2).unwrap(), ctx)?.to_entity(),
                        )))
                        .unwrap(),
                }
                Ok(JsValue::undefined())
            })
        };

        // Js触发Signal的方法
        let signal_emit = unsafe {
            NativeFunction::from_closure(|_referrer, args, ctx| {
                let signal = args.first().unwrap().to_object(ctx)?;

                let signal_args = args[1..].to_owned();
                let connect_array = JsArray::from_object(
                    signal
                        .get(js_string!("connectArray"), ctx)?
                        .to_object(ctx)?,
                )?;

                let func = connect_array
                    .get(js_string!("0"), ctx)?
                    .as_function()
                    .unwrap();
                func.call(&JsValue::Undefined, &signal_args, ctx)?;
                Ok(JsValue::undefined())
            })
        };

        // Js注册由bevy触发的Signal的方法
        let register_default_signal = unsafe {
            NativeFunction::from_closure(|_referrer, args, ctx| {
                let signal = args.first().unwrap().to_object(ctx)?;

                let default_signal_type =
                    JsSignalType::try_from_js(&signal.get(js_string!("type"), ctx)?, ctx)?;

                match default_signal_type {
                    JsSignalType::Created => emit_signal(&signal, &[], ctx)?,
                    JsSignalType::Selected => {
                        let entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?
                                .to_entity();
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<SelectedSignalMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(entity, signal);
                    }
                    JsSignalType::OnUnitEnter => {
                        let entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?
                                .to_entity();
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<OnUnitEnterSignalMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(entity, signal);
                    }
                    JsSignalType::OnUnitExit => {
                        let entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?
                                .to_entity();
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<OnUnitExitSignalMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(entity, signal);
                    }
                    JsSignalType::NewWayPoint => {
                        let entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?
                                .to_entity();
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<NewWayPointSignalMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(entity, signal);
                    }
                    JsSignalType::ActiveWayPointChanged => {
                        let entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?
                                .to_entity();
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<ActiveWayPointChangedSignalMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(entity, signal);
                    }
                    JsSignalType::FixedUpdate => {
                        let entity =
                            JsEntity::try_from_js(&signal.get(js_string!("entity"), ctx)?, ctx)?
                                .to_entity();
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<FixedUpdateSignalMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(entity, signal);
                    }

                    JsSignalType::Custom => {}
                }

                Ok(JsValue::undefined())
            })
        };

        // 将Js的Object与Bevy的Entity绑定的方法
        let register_entity = unsafe {
            let sw_cli_request_sender = sw_cli_request_sender.clone();
            let sw_cli_response_receiver = sw_cli_response_receiver.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                sw_cli_request_sender
                    .send(SwCliRequestEvent::RegisterEntity)
                    .unwrap();
                if let SwCliResponseEvent::RegisteredEntity(entity) =
                    sw_cli_response_receiver.lock().unwrap().recv().unwrap()
                {
                    let js_object = args.get_or_undefined(0).to_object(ctx)?;

                    ctx.realm()
                        .host_defined_mut()
                        .get_mut::<JsObjectMap>()
                        .unwrap()
                        .map
                        .borrow_mut()
                        .insert(entity, js_object.clone());

                    if let Ok(js_proxy_func) = js_object.get(js_string!("getSynchronizeProxy"), ctx)
                    {
                        let js_proxy = JsProxy::from_object(
                            js_proxy_func
                                .as_function()
                                .unwrap()
                                .call(&JsValue::Object(js_object), &[], ctx)?
                                .to_object(ctx)?
                                .clone(),
                        )?;
                        ctx.realm()
                            .host_defined_mut()
                            .get_mut::<JsProxyMap>()
                            .unwrap()
                            .map
                            .borrow_mut()
                            .insert(entity, js_proxy);
                    };
                    Ok(JsValue::Object(
                        JsEntity::from_entity(&entity)
                            .try_into_js(ctx)?
                            .to_object(ctx)?,
                    ))
                } else {
                    Ok(JsValue::undefined())
                }
            })
        };

        let create_quick_ui = unsafe {
            let sw_cli_request_sender = sw_cli_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let quick_ui = args.first().unwrap();
                Ok(JsValue::Undefined)
            })
        };

        let register_signal = unsafe {
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let signal_object = args.get_or_undefined(0).to_object(ctx)?;
                let entity =
                    JsEntity::try_from_js(&signal_object.get(js_string!("entity"), ctx)?, ctx)?
                        .to_entity();
                ctx.realm()
                    .host_defined_mut()
                    .get_mut::<SignalEntityMap>()
                    .unwrap()
                    .map
                    .borrow_mut()
                    .insert(entity, signal_object);
                Ok(JsValue::Undefined)
            })
        };

        let get_object = unsafe {
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let target_entity =
                    JsEntity::try_from_js(args.get_or_undefined(0), ctx)?.to_entity();
                match ctx
                    .realm()
                    .host_defined_mut()
                    .get::<JsObjectMap>()
                    .unwrap()
                    .map
                    .borrow()
                    .get(&target_entity)
                {
                    Some(object) => Ok(JsValue::Object(object.clone())),
                    None => Ok(JsValue::Undefined),
                }
            })
        };

        let get_proxy = unsafe {
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let target_entity =
                    JsEntity::try_from_js(args.get_or_undefined(0), ctx)?.to_entity();
                match ctx
                    .realm()
                    .host_defined_mut()
                    .get::<JsProxyMap>()
                    .unwrap()
                    .map
                    .borrow()
                    .get(&target_entity)
                {
                    Some(object) => Ok(JsValue::Object(object.clone().into())),
                    None => Ok(JsValue::Undefined),
                }
            })
        };

        // TODO
        let alter_target_state = unsafe {
            NativeFunction::from_closure(move |_referrer, _args, _ctx| Ok(JsValue::Undefined))
        };

        let synchronize = unsafe {
            let js_engine_response_sender = js_engine_response_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let entity = JsEntity::try_from_js(args.first().unwrap(), ctx)?.to_entity();
                let object = ctx
                    .realm()
                    .host_defined()
                    .get::<JsObjectMap>()
                    .unwrap()
                    .map
                    .borrow()
                    .get(&entity)
                    .unwrap()
                    .clone();

                match SynchronizeDataType::try_from_js(args.get(1).unwrap(), ctx)? {
                    SynchronizeDataType::Core => js_engine_response_sender
                        .send(JsEngineResponseEvent::synchronize_from_js(
                            SynchronizeData::Core(Core::try_from_js(
                                &JsValue::Object(object),
                                ctx,
                            )?),
                        ))
                        .unwrap(),
                    SynchronizeDataType::Movement => js_engine_response_sender
                        .send(JsEngineResponseEvent::synchronize_from_js(
                            SynchronizeData::Movement(Movement::try_from_js(
                                &JsValue::Object(object),
                                ctx,
                            )?),
                        ))
                        .unwrap(),
                };

                Ok(JsValue::Undefined)
            })
        };

        let bind_inner_info = unsafe {
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                if let Some(object) = args.first() {
                    let object = object.to_object(ctx)?;
                    let entity =
                        JsEntity::try_from_js(&object.get(js_string!("entity"), ctx)?, ctx)?
                            .to_entity();

                    Ok(JsValue::Boolean(true))
                } else {
                    Ok(JsValue::Boolean(false))
                }
            })
        };

        let fs = Fs::init(context, sw_fs_request_sender.clone());

        let server_objects = init_server_objects(context);

        let mut initializer = ObjectInitializer::with_native_data_and_proto(
            Self,
            JsObject::with_object_proto(context.realm().intrinsics()),
            context,
        );
        initializer
            .property(
                JsSymbol::to_string_tag(),
                Self::NAME,
                Attribute::CONFIGURABLE,
            )
            .function(teleport, js_string!("teleport"), 3)
            .function(look_at, js_string!("lookAt"), 3)
            //.function(register_signal, js_string!("register_signal"), 1)
            .function(signal_emit, js_string!("signalEmit"), 2)
            .function(register_entity, js_string!("registerEntity"), 0)
            .function(
                register_default_signal,
                js_string!("registerDefaultSignal"),
                1,
            )
            .function(create_quick_ui, js_string!("createQuickUi"), 1)
            .function(register_signal, js_string!("registerSignal"), 1)
            .function(get_object, js_string!("getObject"), 1)
            .function(get_proxy, js_string!("getProxy"), 1)
            .function(alter_target_state, js_string!("alterTargetState"), 3)
            .function(synchronize, js_string!("synchronize"), 2)
            .function(bind_inner_info, js_string!("bindInnerInfo"), 1)
            .property(Fs::NAME, fs, Attribute::CONFIGURABLE);

        for (server_name, server_object) in server_objects {
            initializer.property(server_name, server_object, Attribute::CONFIGURABLE);
        }

        initializer.build()
    }
}
