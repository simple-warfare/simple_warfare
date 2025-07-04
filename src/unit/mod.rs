pub mod custom_unit;
pub mod physics;
pub mod section;
pub mod way_point;

use crate::{
    js_engine::event::JsEngineResponseEvent,
    scenes::game::input::Selectable,
    unit::{
        custom_unit::CustomUnit, physics::{EnablePhysics, PhysicsPlugin}, section::{
            core::Core,
            graphic::{Graphic, Graphics},
            movement::Movement,
        }
    },
};

use bevy::prelude::*;
use std::path::Path;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewSpawnedUnit>()
            .register_type::<Core>()
            .register_type::<Graphic>()
            .register_type::<Graphics>()
            .register_type::<Movement>()
            .add_plugins(PhysicsPlugin)
            .add_systems(
                Update,
                check_new_unit.run_if(on_event::<JsEngineResponseEvent>),
            );
    }
}

#[derive(Event)]
pub struct NewSpawnedUnit(pub Entity);

fn check_new_unit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<JsEngineResponseEvent>,
    mut writer: EventWriter<NewSpawnedUnit>,
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
                CustomUnit,
                Selectable,
                EnablePhysics,
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
            writer.write(NewSpawnedUnit(*entity));
        }
    }
    Ok(())
}
