use std::sync::{Arc, mpsc::Sender};

use bevy::prelude::*;

use crate::{
    assets::mods::{info::ModInfo, js::JsAsset},
    js_engine::event::JsEngineRequestEvent,
    lua_engine::user_data::ModEnableClasses,
};

#[derive(Resource)]
pub struct ModServer {
    sender: Arc<Sender<JsEngineRequestEvent>>,
}

impl ModServer {
    pub fn new(sender: Arc<Sender<JsEngineRequestEvent>>) -> Self {
        Self { sender }
    }
    pub fn spawn_unit(&self, unit_str: &str) {
        self.sender
            .send(JsEngineRequestEvent::SpawnUnit(unit_str.to_string()))
            .unwrap();
    }

    pub fn load_mod(&self, enables: Vec<(JsAsset, Vec<String>)>, info: ModInfo) -> Result<()> {
        self.sender.send(JsEngineRequestEvent::LoadMod(
            ModEnableClasses::new(enables),
            info.clone(),
        ))?;
        Ok(())
    }
}
