pub mod server;

use std::sync::atomic::Ordering;

use bevy::prelude::*;

use crate::{
    js_engine::{JsEngineRequestSender, JsEngineResponseReciver, event::JsEngineResponseEvent},
    statistics::SomeAsyncWorkCalculator,
};

use self::server::ModServer;

pub struct ModEnginePlugin;

impl Plugin for ModEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            init_mod_server.run_if(resource_exists::<JsEngineRequestSender>.and(run_once)),
        )
        .add_systems(
            Update,
            loaded_custom_units_event.run_if(
                resource_exists::<JsEngineResponseReciver>.and(resource_exists::<ModServer>),
            ),
        );
    }
}

fn init_mod_server(mut commands: Commands, js_engine_event_sender: Res<JsEngineRequestSender>) {
    commands.insert_resource(ModServer::new(js_engine_event_sender.0.clone()));
}

fn loaded_custom_units_event(
    mut reader: EventReader<JsEngineResponseEvent>,
    mut mod_server: ResMut<ModServer>,
    some_async_work_calculator: Res<SomeAsyncWorkCalculator>,
) {
    for event in reader.read() {
        if let JsEngineResponseEvent::LoadedCustomUnits { loaded_number } = event {
            mod_server.loaded_custom_unit_number += loaded_number;
            let loaded_custom_unit_number = mod_server.loaded_custom_unit_number;
            let custom_unit_number = mod_server.custom_unit_number;
            if loaded_custom_unit_number < custom_unit_number {
                info!(
                    "已加载单位数量:{},剩余{}未加载",
                    loaded_custom_unit_number,
                    custom_unit_number - loaded_custom_unit_number
                );
            }

            if loaded_custom_unit_number == custom_unit_number {
                info!("已加载完所有单位,总数量为:{}", custom_unit_number);
                some_async_work_calculator.0.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}
