use bevy::prelude::*;
use lightyear::prelude::*;

use crate::net::shared::SERVER_ADDR;

pub struct ClinetPlugin;

impl Plugin for ClinetPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

fn init_client(mut commnads: Commands) {
}
