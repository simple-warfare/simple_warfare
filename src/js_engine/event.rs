use bevy::prelude::*;

use crate::{
    assets::mods::js::JsAsset,
    custom::{
        CustomModAsset,
        unit::{section::core::Core, unit::SpawnedUnitData},
    },
    js_engine::{
        global::class::entity::JsEntity,
        sw::{LookType, TeleportType},
        synchronize::SynchronizeData,
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
    SynchronizeData(SynchronizeData), //RemoteJsProxy(Box<dyn Fn(JsProxy) -> String + Send + Sync + 'static>),
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

    pub fn emit_empty_signal(signal_entity: Entity) -> Self {
        Self::EmitEmptySignal { signal_entity }
    }
}

#[derive(Debug, Event, Clone)]
pub enum JsEngineResponseEvent {
    EngineInited,
    //Mod
    SpawnedUnit {
        unit_id: UnitId,
        entity: Entity,
        module_path: String,
        data: SpawnedUnitData,
    },
    ToTeleport(TeleportType),
    ToLook(LookType),

    SynchronizeCore(Core),
}

impl JsEngineResponseEvent {
    pub fn spawned_unit(
        unit_id: UnitId,
        entity: Entity,
        module_path: String,
        data: SpawnedUnitData,
    ) -> Self {
        Self::SpawnedUnit {
            unit_id,
            entity,
            module_path,
            data,
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
