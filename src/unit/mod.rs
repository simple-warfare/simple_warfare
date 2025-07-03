pub mod custom_unit;
pub mod section;
pub mod way_point;

use std::path::Path;

use crate::{
    js_engine::event::JsEngineResponseEvent,
    scenes::game::input::Selectable,
    unit::{
        custom_unit::CustomUnit,
        section::{
            core::Core,
            graphic::{Graphic, Graphics}, movement::Movement,
        },
    },
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Core>()
            .register_type::<Graphic>()
            .register_type::<Graphics>()
            .register_type::<Movement>()
            .add_systems(
                Update,
                check_new_unit.run_if(on_event::<JsEngineResponseEvent>),
            );
    }
}

fn check_new_unit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<JsEngineResponseEvent>,
) -> Result {
    for event in reader.read() {
        if let JsEngineResponseEvent::SpawnedUnit(entity, form, spawned_unit_data) = event {
            let core = &spawned_unit_data.core;
            let graphics = &spawned_unit_data.graphics;
            let movement = spawned_unit_data.movement;
            info!("{form}");
            commands.entity(*entity).insert((
                core.clone(),
                graphics.clone(),
                movement,
                Selectable,
                CustomUnit,
                Collider::ball(60.),
                Sprite {
                    image: asset_server.load(
                        Path::new(form)
                            .parent()
                            .unwrap()
                            .join(graphics.data[0].path.clone()),
                    ),
                    ..Default::default()
                },
            ));
        }
    }
    Ok(())
}
