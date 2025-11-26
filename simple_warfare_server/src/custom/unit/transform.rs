use crate::bevy_ext::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use simple_warfare_server_macros::TryFromAndIntoJs;
#[derive(
    Debug, Default, Clone, Component, Serialize, Deserialize, PartialEq, Reflect, TryFromAndIntoJs,
)]
pub struct JsTransform {
    #[boa(
        from_js_with = "option_entity_try_from_js",
        into_js_with = "option_entity_try_into_js"
    )]
    pub entity: Option<Entity>,
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    ///
    /// See the [`translations`] example for usage.
    ///
    /// [`translations`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/translation.rs
    #[boa(from_js_with = "vec3_try_from_js", into_js_with = "vec3_try_into_js")]
    pub translation: Vec3,
    /// Rotation of the entity.
    ///
    /// See the [`3d_rotation`] example for usage.
    ///
    /// [`3d_rotation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs
    #[boa(from_js_with = "quat_try_from_js", into_js_with = "quat_try_into_js")]
    pub rotation: Quat,
    /// Scale of the entity.
    ///
    /// See the [`scale`] example for usage.
    ///
    /// [`scale`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/scale.rs
    #[boa(from_js_with = "vec3_try_from_js", into_js_with = "vec3_try_into_js")]
    pub scale: Vec3,
}

impl JsTransform {
    pub fn update(&mut self, transform: &Transform) {
        self.translation = transform.translation;
        self.scale = transform.scale;
        self.rotation = transform.rotation;
    }
}

impl Into<Transform> for JsTransform {
    fn into(self) -> Transform {
        Transform {
            translation: self.translation,
            rotation: self.rotation,
            scale: self.scale,
        }
    }
}

impl From<Transform> for JsTransform {
    fn from(value: Transform) -> Self {
        Self {
            entity: None,
            translation: value.translation,
            rotation: value.rotation,
            scale: value.scale,
        }
    }
}
