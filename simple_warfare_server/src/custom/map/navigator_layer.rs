pub mod northstar;

use std::sync::atomic::Ordering;

use bevy::prelude::*;

use crate::{
    assets::custom::map::grid_layers::CustomGridLayers,
    custom::map::navigator_layer::northstar::CustomGridLayersServer,
    statistics::{ServerState, SomeAsyncWorkCalculator},
};

pub struct NavigatorLayerPlugin;

impl Plugin for NavigatorLayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ServerState::SomeAsyncWork), load_custom_layers)
            .add_systems(
                Update,
                check_custom_layers_ready.run_if(in_state(ServerState::SomeAsyncWork)),
            );
    }
}

fn load_custom_layers(
    mut custom_grid_layers_server: ResMut<CustomGridLayersServer>,
    asset_server: Res<AssetServer>,
) {
    custom_grid_layers_server.handles = Some(
        custom_grid_layers_server
            .new_layer
            .iter()
            .map(|path| asset_server.load(path.as_path()))
            .collect(),
    );
}

fn check_custom_layers_ready(
    mut laod_ready_calculator: Local<usize>,
    mut events: EventReader<AssetEvent<CustomGridLayers>>,
    custom_grid_layers_assets: Res<Assets<CustomGridLayers>>,
    mut custom_grid_layers_server: ResMut<CustomGridLayersServer>,
    some_async_work_calculator: Res<SomeAsyncWorkCalculator>,
) -> Result {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } = *event else {
            return Ok(());
        };

        if let Some(custom_grid_layers) = custom_grid_layers_assets.get(id) {
            *laod_ready_calculator += 1;
            custom_grid_layers_server.add_layer(custom_grid_layers.clone());
        } else {
            return Err(BevyError::from(
                "couldn't get the custom grid layers asset by id",
            ));
        }

        if *laod_ready_calculator == custom_grid_layers_server.new_layer.len() {
            info!("全部地形类型加载完成:\n{:#?}", custom_grid_layers_server.layer);
            some_async_work_calculator.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    Ok(())
}
