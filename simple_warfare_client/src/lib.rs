pub mod statistics;
pub mod consts;
pub mod adaptor;

use bevy::{app::PluginGroupBuilder, prelude::*};

use self::statistics::StatistcsPlugin;

pub struct SimpleWarfareClientPlugins;

impl PluginGroup for SimpleWarfareClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(StatistcsPlugin);
        group
    }
}
