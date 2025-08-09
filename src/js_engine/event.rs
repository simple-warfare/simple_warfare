use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    assets::mods::js::JsAsset,
    custom::{
        CustomModAsset, CustomTypedId,
        unit::{
            CustomInnerInfo,
            section::{core::Core, movement::Movement},
            unit::JsUnit,
            way_point::WayPoint,
        },
    },
    js_engine::{
        global::class::entity::JsEntity,
        simple_warfare_cli::{LookType, TeleportType},
        synchronize::{SynchronizeDataFromJs, SynchronizeDataFromJsType},
    },
    net::shared::UnitId,
};

#[derive(Event)]
pub enum JsEngineRequestEvent {
    LoadMod(CustomModAsset),
    SpawnUnit {
        unit_id: UnitId,
        unit_str: String,
    },
    ToTeleport(TeleportType),
    ToLook(LookType),
    //Signal
    SelectedSignalEmit,

    OnUnitEnterSignal {
        target_entities: Vec<JsEntity>,
        signal_entity: Entity,
    },
    OnUnitExitSignal {
        target_entities: Vec<JsEntity>,
        signal_entity: Entity,
    },
    EmitEmptySignal {
        signal_entity: Entity,
    },
    NewWayPointSignal {
        way_point: WayPoint,
        signal_entity: Entity,
    },
    SynchronizeFromJs {
        data: SynchronizeDataFromJs,
    }, //RemoteJsProxy(Box<dyn Fn(JsProxy) -> String + Send + Sync + 'static>),

    InsertCustomInnerInfo {
        custom_typed_id: CustomTypedId,
        entity: Entity,
        custom_inner_info: Arc<CustomInnerInfo>,
    },
}

impl JsEngineRequestEvent {
    pub fn spawn_unit(unit_id: UnitId, unit_str: String) -> Self {
        Self::SpawnUnit { unit_id, unit_str }
    }

    pub fn on_unit_enter_signal_entity(
        target_entities: Vec<JsEntity>,
        signal_entity: Entity,
    ) -> Self {
        Self::OnUnitEnterSignal {
            target_entities,
            signal_entity,
        }
    }

    pub fn on_unit_exit_signal_entity(
        target_entities: Vec<JsEntity>,
        signal_entity: Entity,
    ) -> Self {
        Self::OnUnitExitSignal {
            target_entities,
            signal_entity,
        }
    }

    pub fn new_way_point_signal(way_point: WayPoint, signal_entity: Entity) -> Self {
        Self::NewWayPointSignal {
            way_point,
            signal_entity,
        }
    }

    pub fn emit_empty_signal(signal_entity: Entity) -> Self {
        Self::EmitEmptySignal { signal_entity }
    }

    pub fn synchronize_from_js(data: SynchronizeDataFromJs) -> Self {
        Self::SynchronizeFromJs { data }
    }
}

#[derive(Debug, Event, Clone)]
pub enum JsEngineResponseEvent {
    EngineInited,
    //Mod
    SpawnedUnit { js_unit: JsUnit },
    ToTeleport(TeleportType),
    ToLook(LookType),

    SynchronizeCoreFromJs { data: Core },
    SynchronizeMovementFromJs { data: Movement },
}

impl JsEngineResponseEvent {
    pub fn spawned_unit(js_unit: JsUnit) -> Self {
        Self::SpawnedUnit { js_unit }
    }

    pub fn synchronize_from_js(data: SynchronizeDataFromJs) -> Self {
        match data {
            SynchronizeDataFromJs::Core(core) => Self::SynchronizeCoreFromJs { data: core },
            SynchronizeDataFromJs::Movement(movement) => {
                Self::SynchronizeMovementFromJs { data: movement }
            }
        }
    }
}

#[derive(Debug)]
pub enum SwModuleLoaderRequestEvent {
    LoadJsAsset {
        path: String,
        sender: Box<oneshot::Sender<JsAsset>>,
    },
}

impl SwModuleLoaderRequestEvent {
    pub fn load_js_asset(path: String, sender: Box<oneshot::Sender<JsAsset>>) -> Self {
        Self::LoadJsAsset { path, sender }
    }
}
pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JsEngineRequestEvent>()
            .add_event::<JsEngineResponseEvent>();
    }
}
