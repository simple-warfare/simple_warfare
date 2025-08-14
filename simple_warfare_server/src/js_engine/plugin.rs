use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    assets::mods::js::JsAsset,
    bevy_ext::condition::boa_load_js_asset_has_data,
    js_engine::{event::SwModuleLoaderRequestEvent, loader::*},
};

pub struct SwLoaderPlugin;

#[derive(Default, Resource)]
pub struct BoaLoadJsAsset {
    pub map: HashMap<Handle<JsAsset>, Vec<Box<oneshot::Sender<JsAsset>>>>,
}

impl Plugin for SwLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoaLoadJsAsset>()
            .add_systems(
                Update,
                module_receiver_request.run_if(resource_exists::<SwModuleLoaderRequestReceiver>),
            )
            .add_systems(
                Update,
                module_check_js_asset_ready.run_if(boa_load_js_asset_has_data()),
            );
    }
}

pub(super) fn module_receiver_request(
    asset_server: Res<AssetServer>,
    event_receiver: Res<SwModuleLoaderRequestReceiver>,
    mut boa_load_js_asset: ResMut<BoaLoadJsAsset>,
    js_assets: Res<Assets<JsAsset>>,
) -> Result {
    let Ok(module_loader_request_rx) = event_receiver.0.try_lock() else {
        return Ok(());
    };

    let Ok(SwModuleLoaderRequestEvent::LoadJsAsset { path, sender }) =
        module_loader_request_rx.try_recv()
    else {
        return Ok(());
    };

    let file_handle = asset_server.load(path);
    if asset_server.is_loaded(file_handle.id()) {
        sender.send(js_assets.get(file_handle.id()).unwrap().clone())?;
    } else {
        boa_load_js_asset
            .map
            .entry(file_handle)
            .or_default()
            .push(sender);
    }

    Ok(())
}

fn module_check_js_asset_ready(
    asset_server: Res<AssetServer>,
    mut boa_load_js_asset: ResMut<BoaLoadJsAsset>,
    js_assets: Res<Assets<JsAsset>>,
    mut events: EventReader<AssetEvent<JsAsset>>,
) -> Result {
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = *event
            && let Some(mut senders) = boa_load_js_asset
                .map
                .remove(&asset_server.get_id_handle(id).unwrap())
        {
            senders.drain(..).try_for_each::<_, Result>(|sender| {
                sender.send(
                    js_assets
                        .get(id)
                        .ok_or(BevyError::from("Could not get the js asset"))?
                        .clone(),
                )?;
                Ok(())
            })?;
        }
    }
    Ok(())
}
