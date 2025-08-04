use bevy::{platform::collections::HashMap, prelude::*};

use crate::custom::CustomTypedId;

#[derive(Resource, Default)]
pub struct SharedCutomHandleMapping {
    pub map: HashMap<CustomTypedId, SharedCutomHandle>,
}
pub enum SharedCutomHandle {
    TextureAtlasLayout(Handle<TextureAtlasLayout>),
}

pub struct SharedCustomPlugin;

impl Plugin for SharedCustomPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SharedCutomHandleMapping>();
    }
}
