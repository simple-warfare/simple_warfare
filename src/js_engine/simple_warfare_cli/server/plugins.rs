use std::sync::{Arc, mpsc::Sender};

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use super::trick_film_player::{
    SwTrickFilmPlayerRequestEvent, SwTrickFilmPlayerRequestReceiver, TrickFilmRegistions,
};

pub struct TrickFilmPlayerPlugin;

impl Plugin for TrickFilmPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TrickFilmRegistions>().add_systems(
            Update,
            handle_trick_film_player_event
                .run_if(resource_exists::<SwTrickFilmPlayerRequestReceiver>),
        );
    }
}

fn handle_trick_film_player_event(
    asset_server: Res<AssetServer>,
    mut trick_film_players: Query<&mut AnimationPlayer2D>,
    mut trick_registions: ResMut<TrickFilmRegistions>,
    sw_trick_film_player_request_receiver: ResMut<SwTrickFilmPlayerRequestReceiver>,
) -> Result {
    let Ok(event) = sw_trick_film_player_request_receiver
        .0
        .lock()
        .expect("lock js Response receiver error in the system `engine_inited`")
        .try_recv()
    else {
        return Ok(());
    };

    let animation_clip = |real_parent_path: String, trick_film: String, registion: String| {
        format!("{}/{}#{}", real_parent_path, trick_film, registion)
    };

    match event {
        SwTrickFilmPlayerRequestEvent::Play {
            entity,
            real_parent_path,
            trick_film,
            registion,
        } => {
            let clip = animation_clip(real_parent_path, trick_film, registion);
            info!("{}", clip);
            let clip_handle = trick_registions
                .0
                .entry_ref(&clip)
                .or_insert(asset_server.load(&clip));
            let mut animation_player = trick_film_players.get_mut(entity)?;
            animation_player.play(clip_handle.clone()).repeat();
        }
        SwTrickFilmPlayerRequestEvent::Start {
            entity,
            real_parent_path,
            trick_film,
            registion,
        } => {
            let clip = animation_clip(real_parent_path, trick_film, registion);
            info!("{}", clip);
            let clip_handle = trick_registions
                .0
                .entry_ref(&clip)
                .or_insert(asset_server.load(&clip));
            let mut animation_player = trick_film_players.get_mut(entity)?;
            animation_player.start(clip_handle.clone()).repeat();
        }
    }

    Ok(())
}
