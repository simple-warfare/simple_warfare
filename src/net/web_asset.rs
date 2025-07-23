use axum::{
    http::{HeaderMap, header},
    response::IntoResponse,
};
use bevy::prelude::*;
use bevy_defer::{AsyncAccess, AsyncWorld};
use bevy_webgate::{BevyWebServerPlugin, RouterAppExt};

use crate::{
    assets::{
        byte::ByteFile,
        map::{ldtk::LdtkMap, tiled::TiledMap},
    },
    statistics::SelectedMap,
};


pub struct SimpleWarfareWebAssetPlugin;

impl Plugin for SimpleWarfareWebAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyWebServerPlugin).route(
            "/simple_warfare/clinet/get_room_info/thumbnail",
            axum::routing::get(get_thumbnail_from_this),
        );
    }
}


pub(super) async fn get_thumbnail_from_this() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    let mime_type = new_mime_guess::from_path("thumbnail.png")
        .first_or_octet_stream()
        .to_string();

    headers.insert(header::CONTENT_TYPE, mime_type.parse().unwrap());

    let get_thumbnail_id = AsyncWorld.register_system(get_thumbnail);

    let contexts = AsyncWorld
        .spawn_task(async move {
            if let Ok(thumbnail_byte_file_handle) = AsyncWorld.run_system(get_thumbnail_id) {
                let thumbnail_byte_file = AsyncWorld.asset(thumbnail_byte_file_handle.id());

                thumbnail_byte_file
                    .get(|byte_file| byte_file.data.clone())
                    .unwrap_or(vec![])
            } else {
                vec![]
            }
        })
        .await;

    (headers, contexts).into_response()
}

fn get_thumbnail(
    tiled_maps: Res<Assets<TiledMap>>,
    ldtk_maps: Res<Assets<LdtkMap>>,
    selected_map: Res<SelectedMap>,
    asset_server: Res<AssetServer>,
) -> Handle<ByteFile> {
    asset_server.load::<ByteFile>(
        selected_map
            .0
            .get_thumbnail(&tiled_maps, &ldtk_maps)
            .path()
            .unwrap(),
    )
}
