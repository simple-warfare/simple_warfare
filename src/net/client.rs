use bevy::prelude::*;
use lightyear::prelude::*;

use crate::{net::{
    common::client::CommonClient,
    shared::{CLIENT_PORT, SERVER_ADDR, SHARED_SETTINGS},
}, statistics::NetState};

pub struct ClinetPlugin;

impl Plugin for ClinetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(NetState::Client), init_client);
    }
}

pub(super) fn init_client(mut commnads: Commands) {
    info!("init_client");
    let conditioner = LinkConditionerConfig::average_condition();
    commnads.spawn(CommonClient {
        client_id: 1,
        client_port: CLIENT_PORT,
        server_addr: SERVER_ADDR,
        conditioner: Some(RecvLinkConditioner::new(conditioner.clone())),
        shared: SHARED_SETTINGS,
    });
}
