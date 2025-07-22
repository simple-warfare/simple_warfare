use crate::bevy_ext::try_from_js::*;
use bevy::prelude::*;
use boa_engine::value::TryFromJs;
use serde::{Deserialize, Serialize};
#[derive(
    Debug, Default, Clone, Component, Serialize, Deserialize, PartialEq, Reflect, TryFromJs,
)]
pub struct JsTransform {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    ///
    /// See the [`translations`] example for usage.
    ///
    /// [`translations`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/translation.rs
    #[boa(from_js_with = "vec3_try_from_js")]
    pub translation: Vec3,
    /// Rotation of the entity.
    ///
    /// See the [`3d_rotation`] example for usage.
    ///
    /// [`3d_rotation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs
    #[boa(from_js_with = "quat_try_from_js")]
    pub rotation: Quat,
    /// Scale of the entity.
    ///
    /// See the [`scale`] example for usage.
    ///
    /// [`scale`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/scale.rs
    #[boa(from_js_with = "vec3_try_from_js")]
    pub scale: Vec3,
}

impl JsTransform {
    pub fn to_transform(&self) -> Transform {
        Transform {
            translation: self.translation,
            rotation: self.rotation,
            scale: self.scale,
        }
    }
}
