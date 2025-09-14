pub mod prelude;


pub mod adaptor;
pub mod assets;
pub mod protocol;
pub mod shared;

use bevy::prelude::*;

use self::{
    adaptor::message::{MessageDecodeKind, MessageEncodeKind}, prelude::info::ModInfoKind, shared::SharedPlugin
};

pub struct SimpleWarfareSharedPlugin;

impl Plugin for SimpleWarfareSharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SharedPlugin)
            .init_resource::<MessageDecodeKind>()
            .init_resource::<MessageEncodeKind>()
            .init_resource::<ModInfoKind>();
    }
}
