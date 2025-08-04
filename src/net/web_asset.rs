use axum::{
    extract::Path,
    http::{HeaderMap, header},
    response::IntoResponse,
};
use bevy::{ecs::system::SystemId, prelude::*};
use bevy_defer::{AsyncAccess, AsyncWorld};
use bevy_webgate::{BevyWebServerPlugin, RouterAppExt};

use crate::{
    assets::{byte::ByteFile, map::tiled::SimpleWarfareMap},
    statistics::SelectedMap,
};

pub struct SimpleWarfareWebAssetPlugin;

#[derive(Debug, Resource)]
pub struct GetThumbnailSystemId(pub SystemId<(), Handle<ByteFile>>);

impl Plugin for SimpleWarfareWebAssetPlugin {
    fn build(&self, app: &mut App) {
        let get_thumbnail_system_id = app.register_system(get_thumbnail);
        app.insert_resource(GetThumbnailSystemId(get_thumbnail_system_id))
            .add_plugins(BevyWebServerPlugin)
            .route(
                "/simple_warfare/clinet/get_room_info/thumbnail",
                axum::routing::get(get_thumbnail_from_this),
            )
            .route(
                "/simple_warfare/clinet/fetch_file/{*path}",
                axum::routing::get(get_thumbnail_from_this),
            );
    }
}

pub fn fetch_file(Path(file_path): Path<String>) -> impl IntoResponse {}

pub(super) async fn get_thumbnail_from_this() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    let mime_type = new_mime_guess::from_path("thumbnail.png")
        .first_or_octet_stream()
        .to_string();

    headers.insert(header::CONTENT_TYPE, mime_type.parse().unwrap());

    let get_thumbnail_id = AsyncWorld
        .resource::<GetThumbnailSystemId>()
        .get(|id| id.0)
        .unwrap();

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
    simple_warfare_maps: Res<Assets<SimpleWarfareMap>>,
    selected_map: Res<SelectedMap>,
    asset_server: Res<AssetServer>,
) -> Handle<ByteFile> {
    asset_server.load::<ByteFile>(
        simple_warfare_maps
            .get(selected_map.0.id())
            .unwrap()
            .map_thumbnail_handle
            .path()
            .unwrap(),
    )
}
