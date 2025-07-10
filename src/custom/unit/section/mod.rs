use bevy::{ecs::bundle::Bundle, reflect::Reflect};
use boa_engine::{JsResult, js_string, object::builtins::JsProxy, prelude::*, value::TryFromJs};

use crate::custom::unit::{
    light2d::point_light2d::JsPointLight2d,
    physics::collider::JsCollider,
    section::{
        collider::JsColliders,
        core::Core,
        graphic::{Graphic, Graphics},
        light2d::JsPointLights2d,
        movement::Movement,
    },
    turret::{JsTurret, Turrets},
};

pub mod collider;
pub mod core;
pub mod graphic;
pub mod light2d;
pub mod movement;

#[derive(Debug, Default, Clone, Bundle, Reflect)]
pub struct Section {
    pub core: Core,
    pub colliders: JsColliders,
    pub graphics: Graphics,
    pub movement: Movement,
    pub point_lights: JsPointLights2d,
    pub turrets: Turrets,
}

impl Section {
    pub fn new(
        core: Core,
        colliders: JsColliders,
        graphics: Graphics,
        movement: Movement,
        point_lights: JsPointLights2d,
        turrets: Turrets,
    ) -> Self {
        Self {
            core,
            colliders,
            graphics,
            movement,
            point_lights,
            turrets,
        }
    }

    pub fn try_from_proxy(proxy: &JsProxy, context: &mut Context) -> JsResult<Section> {
        Ok(Self {
            core: Core::try_from_js(&proxy.get(js_string!("core"), context)?, context)?,
            colliders: JsColliders::new(Vec::<JsCollider>::try_from_js(
                &JsValue::Object(
                    proxy
                        .get(js_string!("colliders"), context)?
                        .to_object(context)?,
                ),
                context,
            )?),
            graphics: Graphics::new(Vec::<Graphic>::try_from_js(
                &JsValue::Object(
                    proxy
                        .get(js_string!("graphics"), context)?
                        .to_object(context)?,
                ),
                context,
            )?),
            movement: Movement::try_from_js(&proxy.get(js_string!("movement"), context)?, context)?,
            point_lights: JsPointLights2d::new(Vec::<JsPointLight2d>::try_from_js(
                &JsValue::Object(
                    proxy
                        .get(js_string!("pointLights"), context)?
                        .to_object(context)?,
                ),
                context,
            )?),
            turrets: Turrets::new(Vec::<JsTurret>::try_from_js(
                &JsValue::Object(
                    proxy
                        .get(js_string!("turrets"), context)?
                        .to_object(context)?,
                ),
                context,
            )?),
        })
    }
}
