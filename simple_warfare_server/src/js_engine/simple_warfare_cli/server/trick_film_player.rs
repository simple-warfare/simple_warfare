use std::{
    path::Path,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
};

use crate::{bevy_ext::prelude::*, js_engine::simple_warfare_cli::SwCliRequestEvent};
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_trickfilm::prelude::*;
use boa_engine::{
    JsArgs, js_string, object::ObjectInitializer, prelude::*, property::Attribute, value::TryIntoJs,
};

#[derive(Resource)]
pub struct SwTrickFilmPlayerRequestReceiver(
    pub Arc<Mutex<Receiver<SwTrickFilmPlayerRequestEvent>>>,
);

#[derive(Resource, Clone)]
pub struct SwTrickFilmPlayerResponseSender(pub Arc<Sender<SwTrickFilmPlayerResponseEvent>>);

#[derive(Event)]
pub enum SwTrickFilmPlayerRequestEvent {
    Play {
        entity: Entity,
        real_parent_path: String,
        trick_film: String,
        registion: String,
    },
    Start {
        entity: Entity,
        real_parent_path: String,
        trick_film: String,
        registion: String,
    },
}

#[derive(Debug, Resource, Default)]
pub struct TrickFilmRegistions(pub HashMap<String, Handle<AnimationClip2D>>);

#[derive(Event)]
pub enum SwTrickFilmPlayerResponseEvent {}
#[derive(Debug, Default, Trace, Finalize, JsData)]
pub struct TrickFilmPlayerServer;

impl TrickFilmPlayerServer {
    pub const NAME: JsString = js_string!("trickFilmPlayerServer");

    pub fn init(
        context: &mut Context,
        sw_trick_film_player_request_sender: Arc<Sender<SwTrickFilmPlayerRequestEvent>>,
    ) -> JsObject {
        let play = unsafe {
            let sw_trick_film_player_request_sender = sw_trick_film_player_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let Some(trick_film_player) = args.first() else {
                    return Ok(JsValue::Boolean(false));
                };
                let trick_film_player = trick_film_player.to_object(ctx)?;

                sw_trick_film_player_request_sender
                    .send(SwTrickFilmPlayerRequestEvent::Play {
                        entity: entity_try_from_js(
                            &trick_film_player.get(js_string!("entity"), ctx)?,
                            ctx,
                        )?,
                        real_parent_path: args
                            .get_or_undefined(1)
                            .to_string(ctx)?
                            .to_std_string_lossy(),
                        registion: args
                            .get_or_undefined(1)
                            .to_string(ctx)?
                            .to_std_string_lossy(),
                        trick_film: trick_film_player
                            .get(js_string!("trickFilm"), ctx)?
                            .to_string(ctx)?
                            .to_std_string_lossy(),
                    })
                    .unwrap();
                Ok(JsValue::Boolean(true))
            })
        };

        let start = unsafe {
            let sw_trick_film_player_request_sender = sw_trick_film_player_request_sender.clone();
            NativeFunction::from_closure(move |_referrer, args, ctx| {
                let Some(trick_film_player) = args.first() else {
                    return Ok(JsValue::Boolean(false));
                };
                let trick_film_player = trick_film_player.to_object(ctx)?;

                sw_trick_film_player_request_sender
                    .send(SwTrickFilmPlayerRequestEvent::Start {
                        entity: entity_try_from_js(
                            &trick_film_player.get(js_string!("entity"), ctx)?,
                            ctx,
                        )?,
                        real_parent_path: args
                            .get_or_undefined(1)
                            .to_string(ctx)?
                            .to_std_string_lossy(),
                        registion: args
                            .get_or_undefined(2)
                            .to_string(ctx)?
                            .to_std_string_lossy(),
                        trick_film: trick_film_player
                            .get(js_string!("trickFilm"), ctx)?
                            .to_string(ctx)?
                            .to_std_string_lossy(),
                    })
                    .unwrap();
                Ok(JsValue::Boolean(true))
            })
        };

        ObjectInitializer::with_native_data_and_proto(
            Self,
            JsObject::with_object_proto(context.realm().intrinsics()),
            context,
        )
        .property(
            JsSymbol::to_string_tag(),
            Self::NAME,
            Attribute::CONFIGURABLE,
        )
        .function(play, js_string!("play"), 3)
        .function(start, js_string!("start"), 3)
        .build()
    }
}
