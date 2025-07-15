use bevy::prelude::*;

use crate::{
    custom::unit::section::{core::Core, movement::Movement},
    js_engine::{JsEngineRequestSender, event::JsEngineRequestEvent},
};

pub enum SynchronizeData {
    //Section
    Core(Core),
    Movement(Movement),
}

pub struct SynchronizePlugin;

impl Plugin for SynchronizePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, synchronize_data
                    .run_if(resource_exists::<JsEngineRequestSender>));
    }
}

pub fn synchronize_data(
    js_engine_requests_sender: Res<JsEngineRequestSender>,
    synchronize_datas: Query<&Core, Changed<Core>>,
) -> Result {
    for core in synchronize_datas {
        js_engine_requests_sender
            .0
            .send(JsEngineRequestEvent::SynchronizeData(
                SynchronizeData::Core(core.clone()),
            ))?;
    }
    Ok(())
}
