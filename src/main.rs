use bevy::{prelude::*, window::WindowMode};

use bevy_web_asset::WebAssetPlugin;
use simple_warfare::SimpleWarfarePlugin;
fn main() {
    let mut app = App::new();
    app.add_plugins(WebAssetPlugin::default())  //实现从http或https加载文件，可以用来同步或获取房主的文件
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SimpleWarfare".into(),
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SimpleWarfarePlugin)
        .run();
}
