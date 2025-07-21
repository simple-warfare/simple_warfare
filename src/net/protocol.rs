use bevy::prelude::*;
use lightyear::prelude::*;

use crate::custom::unit::section::core::Core;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.register_component::<Core>()
            .add_prediction(PredictionMode::Once)
            .add_interpolation(InterpolationMode::Once);
    }
}
