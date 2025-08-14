use std::sync::{Arc, mpsc::Sender};

use bevy::prelude::*;

use crate::{
    custom::CustomModAsset,
    js_engine::event::JsEngineRequestEvent,
    net::{protocol::ClientMessage, shared::UnitId},
};

#[derive(Resource)]
pub struct ModServer {
    sender: Arc<Sender<JsEngineRequestEvent>>,
    pub client_messages: Vec<ClientMessage>,
}

impl ModServer {
    pub fn new(sender: Arc<Sender<JsEngineRequestEvent>>) -> Self {
        Self {
            sender,
            client_messages: Vec::new(),
        }
    }
    pub fn want_spawn_unit(&mut self, unit_str: &str) {
        self.client_messages
            .push(ClientMessage::spawn_unit(unit_str.to_string()));
    }

    pub fn spawn_unit(&self, unit_id: UnitId, unit_str: impl Into<String>) {
        self.sender
            .send(JsEngineRequestEvent::spawn_unit(unit_id, unit_str.into()))
            .unwrap();
    }

    pub fn load_mod(&self, mod_asset: CustomModAsset) -> Result<()> {
        self.sender.send(JsEngineRequestEvent::LoadMod(mod_asset))?;
        Ok(())
    }
}
