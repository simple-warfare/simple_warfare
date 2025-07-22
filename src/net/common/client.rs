//! This module introduces a settings struct that can be used to configure the server and client.
#![allow(unused_imports)]
#![allow(unused_variables)]
use core::net::{Ipv4Addr, SocketAddr};

use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use lightyear::netcode::NetcodeClient;
use lightyear::netcode::client_plugin::NetcodeConfig;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::net::shared::SharedSettings;

/// Event that examples can trigger to spawn a client.
#[derive(Component, Clone, Debug)]
#[component(on_add = CommonClient::on_add)]
pub struct CommonClient {
    pub client_id: u64,
    /// The client port to listen on
    pub client_port: u16,
    /// The socket address of the server
    pub server_addr: SocketAddr,
    /// Possibly add a conditioner to simulate network conditions
    pub conditioner: Option<RecvLinkConditioner>,
    pub shared: SharedSettings,
}

impl CommonClient {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let entity = context.entity;
        world.commands().queue(move |world: &mut World| -> Result {
            let mut entity_mut = world.entity_mut(entity);
            let settings = entity_mut.take::<CommonClient>().unwrap();
            let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), settings.client_port);
            entity_mut.insert((
                Client::default(),
                Link::new(settings.conditioner.clone()),
                LocalAddr(client_addr),
                PeerAddr(settings.server_addr),
                ReplicationReceiver::default(),
                PredictionManager::default(),
                InterpolationManager::default(),
                Name::from("Client"),
            ));

            let add_netcode = |entity_mut: &mut EntityWorldMut| -> Result {
                // use dummy zeroed key explicitly here.
                let auth = Authentication::Manual {
                    server_addr: settings.server_addr,
                    client_id: settings.client_id,
                    private_key: settings.shared.private_key,
                    protocol_id: settings.shared.protocol_id,
                };
                let netcode_config = NetcodeConfig {
                    // Make sure that the server times out clients when their connection is closed
                    client_timeout_secs: 3,
                    token_expire_secs: -1,
                    ..default()
                };
                entity_mut.insert(NetcodeClient::new(auth, netcode_config)?);
                Ok(())
            };

            add_netcode(&mut entity_mut)?;
            entity_mut.insert(UdpIo::default());
            let client_connect_system = world.register_system_cached(client_connect);
            world.run_system(client_connect_system)?;
            Ok(())
        });
    }
}

pub(crate) fn client_connect(mut commands: Commands, client: Single<Entity, With<Client>>) {
    commands.trigger_targets(Connect, client.into_inner());
}
