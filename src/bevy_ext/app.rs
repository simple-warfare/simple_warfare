use crate::{
    bevy_ext::{
        condition::pressed_button,
        panel::{Panel, PanelState},
        system::despawn_screen,
    },
    scenes::{Scene, SceneState},
};
use bevy::{ecs::system::ScheduleSystem, prelude::*};

pub trait AppExt {
    fn init_scene<T: Scene>(&mut self) -> &mut Self;
    fn add_scene_system<T: Component, M>(
        &mut self,
        states: SceneState,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;
    fn init_panel<T: Panel>(&mut self) -> &mut Self;
    fn add_panel_system<T: Component, M>(
        &mut self,
        states: PanelState,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;
}

impl AppExt for App {
    fn init_scene<T: Scene>(&mut self) -> &mut Self {
        T::default().build(self);
        self
    }
    fn init_panel<T: Panel>(&mut self) -> &mut Self {
        T::default().build(self);
        self
    }

    fn add_scene_system<T: Component, M>(
        &mut self,
        states: SceneState,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.add_systems(OnEnter(states), systems)
            .add_systems(OnExit(states), despawn_screen::<T>)
    }

    fn add_panel_system<T: Component, M>(
        &mut self,
        states: PanelState,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.add_systems(OnEnter(states), systems)
            .add_systems(OnExit(states), despawn_screen::<T>)
            .add_systems(
                Update,
                (
                    despawn_screen::<T>,
                    |mut panel_state: ResMut<NextState<PanelState>>| {
                        panel_state.set(PanelState::None);
                    },
                )
                    .run_if(in_state(states).and(pressed_button(KeyCode::Escape))),
            )
    }
}
