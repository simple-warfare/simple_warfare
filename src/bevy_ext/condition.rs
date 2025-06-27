use bevy::prelude::*;
use std::hash::Hash;

pub fn pressed_button<T>(code: T) -> impl FnMut(Res<ButtonInput<T>>) -> bool + Clone
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    move |input: Res<ButtonInput<T>>| input.just_pressed(code)
}
