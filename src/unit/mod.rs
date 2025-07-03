pub mod custom_unit;
pub mod section;
use std::path::Path;

use crate::{
    js_engine::event::JsEngineResponseEvent,
    unit::section::{
        core::Core,
        graphic::{Graphic, Graphics},
    },
};
use bevy::{prelude::*, render::view::RenderLayers};
use bevy_rapier2d::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Core>()
            .register_type::<Graphic>()
            .register_type::<Graphics>()
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
            let core = spawned_unit_data.core.clone();
            let graphics = spawned_unit_data.graphics.clone();
            info!("{form}");
            commands.entity(*entity).insert((
                core,
                graphics.clone(),
                Collider::ball(30.),
                RenderLayers::layer(1),
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
