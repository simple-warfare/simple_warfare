use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
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
    .add_plugins(SimpleWarfarePlugin)
    .run();
}
