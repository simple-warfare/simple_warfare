pub mod main_menu;
use bevy::prelude::*;

use crate::{
    bevy_ext::app::AppExt,
    panel::main_menu::{MainMenu, MainMenuState},
};

pub trait Panel: Default {
    fn build(&self, app: &mut App);
}

pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>().init_panel::<MainMenu>();
    }
}
