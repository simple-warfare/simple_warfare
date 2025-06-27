use bevy::prelude::*;

pub trait Panel: Default {
    fn build(&self, app: &mut App);
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum PanelState {
    #[default]
    None,
}
