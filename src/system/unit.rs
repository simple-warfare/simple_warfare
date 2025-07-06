use std::path::Path;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    custom_unit::{
        NewSpawnedUnit,
        physics::EnablePhysics,
        section::{
            core::Core,
            graphic::{Graphic, Graphics},
            movement::Movement,
        },
        unit::CustomUnit,
        way_point::WayPointQueue,
    },
    js_engine::event::JsEngineResponseEvent,
    statistics::*,
};

pub struct UnitSystemPlugin;
impl Plugin for UnitSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewSpawnedUnit>()
            .register_type::<Core>()
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
    mut writer: EventWriter<NewSpawnedUnit>,
) -> Result {
    for event in reader.read() {
        if let JsEngineResponseEvent::SpawnedUnit(entity, form, spawned_unit_data) = event {
            let core = &spawned_unit_data.core;
            let graphics = &spawned_unit_data.graphics;
            let movement = spawned_unit_data.movement;
            let colliders = &spawned_unit_data.colliders;
            commands
                .entity(*entity)
                .insert(ComputedMass::new(core.mass));
            commands
                .entity(*entity)
                .insert((
                    Name::new(core.name.clone()),
                    core.clone(),
                    graphics.clone(),
                    colliders.clone(),
                    movement,
                    CustomUnit,
                    Selectable,
                    EnablePhysics,
                    WayPointQueue::default(),
                    ExternalForce::default().with_persistence(false),
                    RigidBody::Dynamic,
                    MaxLinearSpeed(40.),
                    AngularDamping(0.8),
                    LinearDamping(0.8),
                    Sprite {
                        image: asset_server.load(
                            Path::new(form)
                                .parent()
                                .unwrap()
                                .join(graphics.data[0].path.clone()),
                        ),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    for collider in colliders.to_avian2d().iter() {
                        parent.spawn(collider.clone());
                    }
                });
            writer.write(NewSpawnedUnit(*entity));
        }
    }
    Ok(())
}
