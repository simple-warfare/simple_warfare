use bevy::{platform::collections::HashMap, prelude::*};

use crate::custom::CustomTypedId;

pub struct SharedCustomPlugin;

impl Plugin for SharedCustomPlugin {
    fn build(&self, app: &mut App) {}
}
