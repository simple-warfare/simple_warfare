use bevy::{color::palettes::css::*, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    assets::{
        GameAsset,
        texture::{TextureAtlasLayoutHandles, interface::DialogTextureSlicer},
    },
    bevy_ext::app::AppExt,
};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct SelectMapScene;

#[derive(Component)]
struct SelectMapSceneMark;

impl Scene for SelectMapScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<SelectMapSceneMark, _, _>(SceneState::SelectMapScene, setup);
    }
}

fn setup(mut commands: Commands, game_asset: Res<GameAsset>) {
    commands.spawn((
        SelectMapSceneMark,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_self: JustifySelf::Center,
            align_items: AlignItems::End,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        BackgroundColor(Color::Srgba(GRAY)),
    ));
}
