use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    assets::mods::{info::{ModEnable, ModInfo}, js::JsAsset},
    js_engine::event::{JsEngineEvent, ModEvent},
};
use tokio::sync::mpsc::{self, UnboundedReceiver as Receiver, UnboundedSender as Sender};

#[derive(Resource)]
pub struct ModServer {
    sender: Arc<Sender<JsEngineEvent>>,
}

impl ModServer {
    pub fn new(sender: Arc<Sender<JsEngineEvent>>) -> Self {
        Self { sender }
    }
    pub fn spawn_unit(&self, entity: Entity, unit_str: &str) -> Entity {
        self.sender
            .send(JsEngineEvent::ModEvent(ModEvent::SpawnUnit(
                entity,
                unit_str.to_string(),
            )))
            .unwrap();
        entity
    }

    pub fn load_mod(&self, enables: Vec<(JsAsset, Vec<String>)>, info: ModInfo) -> Result<()> {
        self.sender.send(JsEngineEvent::ModEvent(ModEvent::LoadMod(
            ModEnable::new(enables),
            info.clone(),
        )))?;
        Ok(())
    }
}
