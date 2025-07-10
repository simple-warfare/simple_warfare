pub mod quick;

use bevy::prelude::*;

use crate::custom::ui::quick::CustomQuickUiPlugin;

pub struct CustomUiPlugin;

impl Plugin for CustomUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CustomQuickUiPlugin);
    }
}
