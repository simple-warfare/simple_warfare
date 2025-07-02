use std::sync::{Arc, mpsc::Sender};

use bevy::prelude::*;

use crate::{
    assets::mods::{
        info::{ModEnable, ModInfo},
        js::JsAsset,
    },
    js_engine::event::JsEngineRequestEvent,
};

#[derive(Resource)]
pub struct ModServer {
    sender: Arc<Sender<JsEngineRequestEvent>>,
}

impl ModServer {
    pub fn new(sender: Arc<Sender<JsEngineRequestEvent>>) -> Self {
        Self { sender }
    }
    pub fn spawn_unit(&self, entity: Entity, unit_str: &str) -> Entity {
        self.sender
            .send(JsEngineRequestEvent::SpawnUnit(
                entity,
                unit_str.to_string(),
            ))
            .unwrap();
        entity
    }

    pub fn load_mod(&self, enables: Vec<(JsAsset, Vec<String>)>, info: ModInfo) -> Result<()> {
        self.sender.send(JsEngineRequestEvent::LoadMod(
            ModEnable::new(enables),
            info.clone(),
        ))?;
        Ok(())
    }
}
