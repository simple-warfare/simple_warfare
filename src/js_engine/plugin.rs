use bevy::prelude::*;

use crate::{assets::mods::js::JsAsset, js_engine::loader::*};

pub struct SwModuleLoaderPlugin;

impl Plugin for SwModuleLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<JsAssetHandles>()
            .add_systems(
                Update,
                receiver_request.run_if(resource_exists::<SwModuleLoaderRequestReceiver>),
            )
            .add_systems(
                Update,
                check_js_asset_ready.run_if(resource_exists::<SwModuleLoaderResponeSender>),
            );
    }
}

#[derive(Resource, Default)]
pub struct JsAssetHandles(pub Vec<Handle<JsAsset>>);

pub(super) fn receiver_request(
    asset_server: Res<AssetServer>,
    event_receiver: Res<SwModuleLoaderRequestReceiver>,
    mut js_asset_handles: ResMut<JsAssetHandles>,
    js_assets: Res<Assets<JsAsset>>,
    sender: Res<SwModuleLoaderResponeSender>,
) -> Result {
    if let Ok(SwModuleLoaderRequestEvent::LoadJsAsset(path)) =
        event_receiver.0.lock().unwrap().try_recv()
    {
        info!("try load module:{}", path);
        let asset = asset_server.load(path);
        if asset_server.is_loaded_with_dependencies(asset.id()) {
            sender
                .0
                .send(SwModuleLoaderResponeEvent::LoadedJsAsset(
                    js_assets.get(asset.id()).unwrap().clone(),
                ))
                .unwrap();
        } else {
            js_asset_handles.0.push(asset);
        }
    }

    Ok(())
}

fn check_js_asset_ready(
    asset_server: Res<AssetServer>,
    js_asset_handles: Res<JsAssetHandles>,
    js_assets: Res<Assets<JsAsset>>,
    mut events: EventReader<AssetEvent<JsAsset>>,
    sender: Res<SwModuleLoaderResponeSender>,
) -> Result {
    for event in events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } => {
                if js_asset_handles
                    .0
                    .contains(&asset_server.get_id_handle(*id).unwrap())
                {
                    info!("加载完成");
                    sender.0.send(SwModuleLoaderResponeEvent::LoadedJsAsset(
                        js_assets.get(*id).unwrap().clone(),
                    ))?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
