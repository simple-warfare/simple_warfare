use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;
use boa_engine::{JsResult, js_string, prelude::*, value::TryFromJs};

#[derive(Debug, Clone, Component, Reflect)]
pub struct JsPointLight2d {
    /// The light's color tint.
    pub color: Color,
    /// The intensity of the light. The light's attenutation is multiplied by this value.
    /// The higher the intensity, the brighter the light.
    pub intensity: f32,
    /// The radius of the light. Illumination will only occur within the light's radius.
    pub radius: f32,
    /// How quickly illumination from the light should deteriorate over distance.
    /// A higher falloff value will result in less illumination at the light's maximum radius.
    pub falloff: f32,
    /// Whether the light should cast shadows.
    pub cast_shadows: bool,
}

impl Default for JsPointLight2d {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.0,
            radius: 0.5,
            falloff: 0.0,
            cast_shadows: false,
        }
    }
}

impl TryFromJs for JsPointLight2d {
    fn try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Self> {
        let point_light2d_object = value.to_object(context)?;
        
        Ok(Self::default())
    }
}

impl JsPointLight2d {
    pub fn to_point_light2d(&self) -> PointLight2d {
        PointLight2d {
            color: self.color,
            intensity: self.intensity,
            radius: self.radius,
            falloff: self.falloff,
            cast_shadows: self.cast_shadows,
        }
    }
}
