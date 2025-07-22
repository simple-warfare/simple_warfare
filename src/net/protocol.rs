use crate::custom::unit::section::core::Core;
use avian2d::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.register_component::<Core>()
            .add_prediction(PredictionMode::Once)
            .add_interpolation(InterpolationMode::Once);

        app.register_component::<Position>()
            .add_prediction(PredictionMode::Full)
            .add_should_rollback(position_should_rollback)
            .add_interpolation(InterpolationMode::Full)
            .add_linear_interpolation_fn()
            .add_linear_correction_fn();

        app.register_component::<Rotation>()
            .add_prediction(PredictionMode::Full)
            .add_should_rollback(rotation_should_rollback)
            .add_interpolation(InterpolationMode::Full)
            .add_linear_interpolation_fn()
            .add_linear_correction_fn();

        // NOTE: interpolation/correction is only needed for components that are visually displayed!
        // we still need prediction to be able to correctly predict the physics on the client
        app.register_component::<LinearVelocity>()
            .add_prediction(PredictionMode::Full);

        app.register_component::<AngularVelocity>()
            .add_prediction(PredictionMode::Full);
    }
}



fn position_should_rollback(this: &Position, that: &Position) -> bool {
    (this.0 - that.0).length() >= 0.01
}

fn rotation_should_rollback(this: &Rotation, that: &Rotation) -> bool {
    this.angle_between(*that) >= 0.01
}