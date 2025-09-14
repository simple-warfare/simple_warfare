use std::sync::{Arc, mpsc::Sender};

use bevy::prelude::*;

use crate::{custom::CustomModAsset, js_engine::event::JsEngineRequestEvent};

#[derive(Resource)]
pub struct ModServer {
    pub custom_unit_number: usize,
    pub loaded_custom_unit_number: usize,
    sender: Arc<Sender<JsEngineRequestEvent>>,
}

impl ModServer {
    pub fn new(sender: Arc<Sender<JsEngineRequestEvent>>) -> Self {
        Self {
            sender,
            custom_unit_number: 0,
            loaded_custom_unit_number: 0,
        }
    }
    pub fn spawn_unit(&self, unit_str: &str) {
        self.sender
            .send(JsEngineRequestEvent::spawn_unit(unit_str.to_string()))
            .unwrap();
    }

    pub fn load_mod(&mut self, mod_asset: CustomModAsset) -> Result<()> {
        mod_asset
            .custom_mod_enable_js
            .iter()
            .for_each(|enable_js| self.custom_unit_number += enable_js.enable_class.len());
        self.sender.send(JsEngineRequestEvent::LoadMod(mod_asset))?;
        Ok(())
    }
}
