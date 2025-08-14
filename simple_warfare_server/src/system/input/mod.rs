pub mod unit;

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::system::input::unit::UnitInputSystemPlugin;

pub struct InputSystemPlugin;

impl Plugin for InputSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_plugins((UnitInputSystemPlugin));
    }
}
