use crate::js_engine::{
    JsEngineEventRequestSender, event::JsEngineRequestEvent, global::class::entity::JsEntity,
};
use bevy::prelude::*;
use bevy_hui::prelude::*;
use boa_engine::{JsResult, js_string, prelude::*, value::TryFromJs};

pub mod html_path {}

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

pub struct CustomQuickUiPlugin;

impl Plugin for CustomQuickUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_quick_ui);
    }
}

fn setup_quick_ui(mut html_funcs: HtmlFunctions) {
    html_funcs.register(
        "quick_ui_comfirm_dialog_comfirm",
        quick_ui_comfirm_dialog_comfirm,
    );

    html_funcs.register(
        "quick_ui_comfirm_dialog_cancel",
        quick_ui_comfirm_dialog_cancel,
    );
}

fn quick_ui_comfirm_dialog_comfirm(
    In(entity): In<Entity>,
    button_tags: Query<&Tags>,
    dialog_query: Query<&QuickComfirmDialog>,
    js_engine_request_sender: Res<JsEngineEventRequestSender>,
) {
    let Some(node_entity) = button_tags.get(entity).ok().and_then(|tags| {
        tags.get("node_entity").and_then(|node_entity_str| {
            Some(
                serde_json::from_str::<Entity>(&node_entity_str)
                    .expect("couldn't parse the entity in html's tags"),
            )
        })
    }) else {
        return;
    };

    let Ok(dialog) = dialog_query.get(node_entity) else {
        return;
    };

    js_engine_request_sender
        .0
        .send(JsEngineRequestEvent::EmitEmptySignal(
            dialog.on_press_comfirm_signal.clone(),
        ))
        .unwrap();
}

fn quick_ui_comfirm_dialog_cancel(
    In(entity): In<Entity>,
    mut commands:Commands,
    button_tags: Query<&Tags>,
    dialog_query: Query<&QuickComfirmDialog>,
    js_engine_request_sender: Res<JsEngineEventRequestSender>,
) {
    let Some(node_entity) = button_tags.get(entity).ok().and_then(|tags| {
        tags.get("node_entity").and_then(|node_entity_str| {
            Some(
                serde_json::from_str::<Entity>(&node_entity_str)
                    .expect("couldn't parse the entity in html's tags"),
            )
        })
    }) else {
        return;
    };

    let Ok(dialog) = dialog_query.get(node_entity) else {
        return;
    };

    js_engine_request_sender
        .0
        .send(JsEngineRequestEvent::EmitEmptySignal(
            dialog.on_press_cancel_signal.clone(),
        ))
        .unwrap();

    commands.entity(node_entity).despawn();
    
}
