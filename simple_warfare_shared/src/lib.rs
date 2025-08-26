pub mod protocol;
pub mod shared;

pub mod prelude {
    pub use crate::protocol::*;
    pub use crate::shared::*;
}
use bevy::prelude::*;

use self::{prelude::ProtocolPlugin, shared::SharedPlugin};

pub struct SimpleWarfareSharedPlugin;

impl Plugin for SimpleWarfareSharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin).add_plugins(SharedPlugin);
    }
}
