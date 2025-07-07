use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;
use boa_engine::value::TryFromJs;

use crate::bevy_ext::try_from_js::*;
use crate::custom_unit::transform::transform::JsTransform;
#[derive(Debug, Clone, Component, Reflect, TryFromJs)]
pub struct JsPointLight2d {
    pub transform: JsTransform,
    /// The light's color tint.
    #[boa(from_js_with = "color_try_from_js")]
    pub color: Color,
    /// The intensity of the light. The light's attenutation is multiplied by this value.
    /// The higher the intensity, the brighter the light.
    #[boa(from_js_with = "f32_try_from_js")]
    pub intensity: f32,
    /// The radius of the light. Illumination will only occur within the light's radius.
    #[boa(from_js_with = "f32_try_from_js")]
    pub radius: f32,
    /// How quickly illumination from the light should deteriorate over distance.
    /// A higher falloff value will result in less illumination at the light's maximum radius.
    #[boa(from_js_with = "f32_try_from_js")]
    pub falloff: f32,
    /// Whether the light should cast shadows.
    pub cast_shadows: bool,
}

impl Default for JsPointLight2d {
    fn default() -> Self {
        Self {
            transform: JsTransform::default(),
            color: Color::WHITE,
            intensity: 1.0,
            radius: 0.5,
            falloff: 0.0,
            cast_shadows: false,
        }
    }
}
impl JsPointLight2d {
    pub fn to_point_light2d(&self) -> impl Bundle {
        (
            self.transform.to_transform(),
            PointLight2d {
                color: self.color,
                intensity: self.intensity,
                radius: self.radius,
                falloff: self.falloff,
                cast_shadows: self.cast_shadows,
            },
        )
    }
}
