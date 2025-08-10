use bevy::prelude::*;
use boa_engine::object::builtins::JsProxy;
use serde::{Deserialize, Serialize};

use crate::bevy_ext::try_from_js::*;
use crate::{
    custom::{
        CustomTypedId,
        unit::{
            section::Section,
            way_point::WayPointQueue,
        },
    },
    net::shared::UnitId,
};
use boa_engine::{JsResult, js_string, prelude::*};

#[derive(Debug, Default, Component)]
pub struct Custom;

#[derive(Debug, Default, Component)]
#[require(Custom, InheritedVisibility)]
pub struct CustomTurrrt;

#[derive(Debug, Default, Component)]
#[require(Custom, InheritedVisibility, WayPointQueue)]
pub struct CustomUnit;


#[derive(Debug, Clone, Bundle, Serialize, Deserialize, Reflect)]

pub struct JsUnit {
    pub section: Section,
    pub data: JsUnitData,
}

#[derive(Debug, Clone, Component, Serialize, Deserialize, Reflect)]
pub struct JsUnitData {
    pub unit_id: UnitId,
    pub custom_typed_id: CustomTypedId,
    pub entity: Entity,
    pub module_path: String,
    pub new_way_point_entity: Entity,
}

impl JsUnit {
    pub fn try_from_proxy(
        proxy: &JsProxy,
        context: &mut Context,
        unit_id: UnitId,
        custom_typed_id: CustomTypedId,
        module_path: String,
    ) -> JsResult<Self> {
        Ok(Self {
            section: Section::try_from_proxy(proxy, context)?,
            data: JsUnitData {
                unit_id,
                custom_typed_id,
                entity: entity_try_from_js(&proxy.get(js_string!("entity"), context)?, context)?,
                module_path,
                new_way_point_entity: entity_try_from_js(
                    &proxy.get(js_string!("newWayPointEntity"), context)?,
                    context,
                )?,
            },
        })
    }
}
