use crate::js_engine::global::class::entity::JsEntity;
use bevy::prelude::*;
use boa_engine::{JsResult, js_string, prelude::*, value::TryFromJs};

#[derive(Clone, PartialEq, Eq, Hash, Component, Reflect)]
pub enum QuickUi {
    Dialog(QuickDialogData),
}

#[derive(Clone, PartialEq, Eq, Hash, Component, Reflect)]
pub enum QuickDialogData {
    Comfirm(QuickComfirmDialog),
}

#[derive(Clone, PartialEq, Eq, Hash, Component, Reflect)]
pub struct QuickComfirmDialog {
    pub title: String,
    pub context: String,
    pub on_press_cancel_signal: JsEntity,
    pub on_press_comfirm_signal: JsEntity,
}

impl QuickComfirmDialog {
    pub fn new(
        title: &str,
        context: &str,
        on_press_cancel_signal: JsEntity,
        on_press_comfirm_signal: JsEntity,
    ) -> Self {
        Self {
            title: title.to_string(),
            context: context.to_string(),
            on_press_cancel_signal,
            on_press_comfirm_signal,
        }
    }
}

impl TryFromJs for QuickUi {
    fn try_from_js(value: &JsValue, context: &mut Context) -> JsResult<Self> {
        let quick_ui_object = value.to_object(context)?;
        match quick_ui_object
            .get(js_string!("type"), context)?
            .to_string(context)?
            .to_std_string_lossy()
            .as_str()
        {
            "Comfirm" => Ok(Self::Dialog(QuickDialogData::Comfirm(
                QuickComfirmDialog::new(
                    &quick_ui_object
                        .get(js_string!("title"), context)?
                        .to_string(context)?
                        .to_std_string_lossy(),
                    &quick_ui_object
                        .get(js_string!("context"), context)?
                        .to_string(context)?
                        .to_std_string_lossy(),
                    JsEntity::try_from_js(
                        &quick_ui_object
                            .get(js_string!("onPressCancel"), context)?
                            .to_object(context)?
                            .get(js_string!("entity"), context)?,
                        context,
                    )?,
                    JsEntity::try_from_js(
                        &quick_ui_object
                            .get(js_string!("onPressComfirm"), context)?
                            .to_object(context)?
                            .get(js_string!("entity"), context)?,
                        context,
                    )?,
                ),
            ))),
            _ => Err(JsNativeError::typ()
                .with_message("the quick ui type is undefine")
                .into()),
        }
    }
}
