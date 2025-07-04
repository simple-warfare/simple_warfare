use bevy::prelude::*;

use crate::{
    assets::mods::js::JsAsset,
    js_engine::{
        event::{SwModuleLoaderRequestEvent, SwModuleLoaderResponseEvent, SwRequireLoaderRequestEvent, SwRequireLoaderResponseEvent},
        loader::*,
    },
};

pub struct SwLoaderPlugin;

impl Plugin for SwLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModuleJsAssetHandles>()
            .init_resource::<RequireJsAssetHandles>()
            .add_systems(
                Update,
                (module_receiver_request, require_receiver_request).run_if(
                    resource_exists::<SwModuleLoaderRequestReceiver>
                        .and(resource_exists::<SwRequireLoaderRequestReceiver>),
                ),
            )
            .add_systems(
                Update,
                (module_check_js_asset_ready, require_check_js_asset_ready).run_if(
                    resource_exists::<SwModuleLoaderResponseSender>
                        .and(resource_exists::<SwRequireLoaderResponseSender>),
                ),
            );
    }
}

#[derive(Resource, Default)]
pub struct ModuleJsAssetHandles(pub Vec<Handle<JsAsset>>);
#[derive(Resource, Default)]
pub struct RequireJsAssetHandles(pub Vec<Handle<JsAsset>>);

pub(super) fn module_receiver_request(
    asset_server: Res<AssetServer>,
    event_receiver: Res<SwModuleLoaderRequestReceiver>,
    mut js_asset_handles: ResMut<ModuleJsAssetHandles>,
    js_assets: Res<Assets<JsAsset>>,
    sender: Res<SwModuleLoaderResponseSender>,
) -> Result {
    if let Ok(SwModuleLoaderRequestEvent::LoadJsAsset(path)) =
        event_receiver.0.lock().unwrap().try_recv()
    {
        let asset = asset_server.load(path);
        if asset_server.is_loaded_with_dependencies(asset.id()) {
            sender
                .0
                .send(SwModuleLoaderResponseEvent::LoadedJsAsset(
                    js_assets.get(asset.id()).unwrap().clone(),
                ))
                .unwrap();
        } else {
            js_asset_handles.0.push(asset);
        }
    }

    Ok(())
}

fn module_check_js_asset_ready(
    asset_server: Res<AssetServer>,
    js_asset_handles: Res<ModuleJsAssetHandles>,
    js_assets: Res<Assets<JsAsset>>,
    mut events: EventReader<AssetEvent<JsAsset>>,
    sender: Res<SwModuleLoaderResponseSender>,
) -> Result {
    for event in events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } => {
                if js_asset_handles
                    .0
                    .contains(&asset_server.get_id_handle(*id).unwrap())
                {
                    sender.0.send(SwModuleLoaderResponseEvent::LoadedJsAsset(
                        js_assets.get(*id).unwrap().clone(),
                    ))?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub(super) fn require_receiver_request(
    asset_server: Res<AssetServer>,
    event_receiver: Res<SwRequireLoaderRequestReceiver>,
    mut js_asset_handles: ResMut<RequireJsAssetHandles>,
    js_assets: Res<Assets<JsAsset>>,
    sender: Res<SwRequireLoaderResponseSender>,
) -> Result {
    if let Ok(SwRequireLoaderRequestEvent::LoadJsAsset(path)) =
        event_receiver.0.lock().unwrap().try_recv()
    {
        let asset = asset_server.load(path);
        if asset_server.is_loaded_with_dependencies(asset.id()) {
            sender
                .0
                .send(SwRequireLoaderResponseEvent::LoadedJsAsset(
                    js_assets.get(asset.id()).unwrap().clone(),
                ))
                .unwrap();
        } else {
            js_asset_handles.0.push(asset);
        }
    }

    Ok(())
}

fn require_check_js_asset_ready(
    asset_server: Res<AssetServer>,
    js_asset_handles: Res<RequireJsAssetHandles>,
    js_assets: Res<Assets<JsAsset>>,
    mut events: EventReader<AssetEvent<JsAsset>>,
    sender: Res<SwRequireLoaderResponseSender>,
) -> Result {
    for event in events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } => {
                if js_asset_handles
                    .0
                    .contains(&asset_server.get_id_handle(*id).unwrap())
                {
                    sender.0.send(SwRequireLoaderResponseEvent::LoadedJsAsset(
                        js_assets.get(*id).unwrap().clone(),
                    ))?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
