use bevy::prelude::*;
use std::hash::Hash;

use crate::mod_engine::server::ModServer;

pub fn pressed_button<T>(code: T) -> impl FnMut(Res<ButtonInput<T>>) -> bool + Clone
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    move |input: Res<ButtonInput<T>>| input.just_pressed(code)
}

pub fn mod_server_has_data() -> impl FnMut(Res<ModServer>) -> bool + Clone {
    move |mod_server: Res<ModServer>| !mod_server.client_messages.is_empty()
}
