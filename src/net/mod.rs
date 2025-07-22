pub mod client;
pub mod common;
pub mod protocol;
pub mod server;
pub mod shared;
use std::time::Duration;

use axum::Json;
use axum::http::{HeaderMap, header};
use axum::response::IntoResponse;
use axum::routing::get;
use bevy::prelude::*;

use bevy_defer::{AsyncAccess, AsyncWorld};
use lightyear::prelude::client::ClientPlugins;
use lightyear::prelude::server::ServerPlugins;

use crate::assets::byte::ByteFile;
use crate::assets::map::ldtk::LdtkMap;
use crate::assets::map::tiled::TiledMap;
use crate::net::client::ClinetPlugin;
use crate::net::server::ServerPlugin;
use crate::net::shared::{FIXED_TIMESTEP_HZ, SharedPlugin};
use crate::statistics::SelectedMap;
use bevy_webgate::{BevyWebServerPlugin, RouterAppExt};
pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyWebServerPlugin)
            .route(
                "/simple_warfare/clinet/get_room_info/thumbnail",
                axum::routing::get(get_thumbnail_from_this),
            )
            .add_plugins(ClientPlugins {
                tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
            })
            .add_plugins(ServerPlugins {
                tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
            })
            .add_plugins((SharedPlugin, ClinetPlugin, ServerPlugin));
    }
}

async fn get_thumbnail_from_this() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    let mime_type = new_mime_guess::from_path("thumbnail.png")
        .first_or_octet_stream()
        .to_string();

    headers.insert(header::CONTENT_TYPE, mime_type.parse().unwrap());

    let get_thumbnail_id = AsyncWorld.register_system(get_thumbnail);

    let contexts = AsyncWorld
        .spawn_task(async move {
            let thumbnail_byte_file_handle = AsyncWorld.run_system(get_thumbnail_id).unwrap();
            let thumbnail_byte_file = AsyncWorld.asset(thumbnail_byte_file_handle.id());

            if thumbnail_byte_file.loaded().await {
                thumbnail_byte_file
                    .get(|byte_file| byte_file.data.clone())
                    .unwrap()
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
