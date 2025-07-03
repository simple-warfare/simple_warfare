use bevy::{
    prelude::*,
    window::WindowMode,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier2d::prelude::*;
use simple_warfare::SimpleWarfarePlugin;
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SimpleWarfare".into(),
                    resolution: (1024.0, 768.0).into(),
                    resizable: false,
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    })
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins((
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0),
        RapierDebugRenderPlugin::default(),
    ))
    .add_plugins(SimpleWarfarePlugin)
    .run();
}

/*
fn set_embedded_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Single<Entity, With<PrimaryWindow>>,
) {
    const ICON_DATA: &[u8] = include_bytes!("../assets/texture/icons/logo.png");
    let image = image::load_from_memory(ICON_DATA)
        .expect("图标解码失败")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    let icon = Icon::from_rgba(rgba, width, height).unwrap();
    if let Some(win) = windows.get_window(*primary_window) {
        win.set_window_icon(Some(icon));
    }
}
*/
