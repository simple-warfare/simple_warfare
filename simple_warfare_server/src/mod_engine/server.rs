use std::sync::{Arc, mpsc::Sender};

use bevy::prelude::*;

use crate::{custom::CustomModAsset, js_engine::event::JsEngineRequestEvent};

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
            .send(JsEngineRequestEvent::spawn_unit(unit_str.to_string()))
            .unwrap();
    }

    pub fn load_mod(&self, mod_asset: CustomModAsset) -> Result<()> {
        self.sender.send(JsEngineRequestEvent::LoadMod(mod_asset))?;
        Ok(())
    }
}
