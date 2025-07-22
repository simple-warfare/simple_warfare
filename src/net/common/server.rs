//! This module introduces a settings struct that can be used to configure the server and client.
#![allow(unused_imports)]
#![allow(unused_variables)]
use core::net::{Ipv4Addr, SocketAddr};

use bevy::asset::ron;
use bevy::prelude::*;
use core::time::Duration;

use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::tasks::IoTaskPool;
use lightyear::netcode::{NetcodeServer, PRIVATE_KEY_BYTES};
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::net::shared::SharedSettings;

#[derive(Component, Debug)]
#[component(on_add = CommonServer::on_add)]
pub struct CommonServer {
    /// Possibly add a conditioner to simulate network conditions
    pub conditioner: Option<RecvLinkConditioner>,
    /// Which transport to use
    pub local_port: u16,
    pub shared: SharedSettings,
}

impl CommonServer {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let entity = context.entity;
        world.commands().queue(move |world: &mut World| -> Result {
            let mut entity_mut = world.entity_mut(entity);
            let settings = entity_mut.take::<CommonServer>().unwrap();
            entity_mut.insert((Name::from("Server"),));

            let add_netcode = |entity_mut: &mut EntityWorldMut| {
                // Use private key from environment variable, if set. Otherwise from settings file.
                let private_key = if let Some(key) = parse_private_key_from_env() {
                    info!("Using private key from LIGHTYEAR_PRIVATE_KEY env var");
                    key
                } else {
                    settings.shared.private_key
                };
                entity_mut.insert(NetcodeServer::new(NetcodeConfig {
                    protocol_id: settings.shared.protocol_id,
                    private_key,
                    ..Default::default()
                }));
            };

            add_netcode(&mut entity_mut);
            let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), settings.local_port);
            entity_mut.insert((LocalAddr(server_addr), ServerUdpIo::default()));

            let server_start_system = world.register_system_cached(server_start);
            world.run_system(server_start_system)?;
            Ok(())
        });
    }
}

pub(crate) fn server_start(mut commands: Commands, server: Single<Entity, With<Server>>) {
    commands.trigger_targets(Start, server.into_inner());
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WebTransportCertificateSettings {
    /// Generate a self-signed certificate, with given SANs list to add to the certifictate
    /// eg: ["example.com", "*.gameserver.example.org", "10.1.2.3", "::1"]
    AutoSelfSigned(Vec<String>),
    /// Load certificate pem files from disk
    FromFile {
        /// Path to cert .pem file
        cert: String,
        /// Path to private key .pem file
        key: String,
    },
}

impl Default for WebTransportCertificateSettings {
    fn default() -> Self {
        let sans = vec![
            "localhost".to_string(),
            "127.0.0.1".to_string(),
            "::1".to_string(),
        ];
        WebTransportCertificateSettings::AutoSelfSigned(sans)
    }
}

/// Reads and parses the LIGHTYEAR_PRIVATE_KEY environment variable into a private key.
pub fn parse_private_key_from_env() -> Option<[u8; PRIVATE_KEY_BYTES]> {
    let Ok(key_str) = std::env::var("LIGHTYEAR_PRIVATE_KEY") else {
        return None;
    };
    let private_key: Vec<u8> = key_str
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ',')
        .collect::<String>()
        .split(',')
        .map(|s| {
            s.parse::<u8>()
                .expect("Failed to parse number in private key")
        })
        .collect();

    if private_key.len() != PRIVATE_KEY_BYTES {
        panic!("Private key must contain exactly {PRIVATE_KEY_BYTES} numbers",);
    }

    let mut bytes = [0u8; PRIVATE_KEY_BYTES];
    bytes.copy_from_slice(&private_key);
    Some(bytes)
}
