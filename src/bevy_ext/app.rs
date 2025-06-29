use crate::{
    bevy_ext::{condition::pressed_button, system::despawn_screen},
    panel::Panel,
    scenes::{Scene, SceneState},
};
use bevy::{ecs::system::ScheduleSystem, prelude::*};

pub trait AppExt {
    fn init_scene<T: Scene>(&mut self) -> &mut Self;
    fn add_scene_system<T: Component, S: States + Copy, M>(
        &mut self,
        states: S,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self;
    fn init_panel<T: Panel>(&mut self) -> &mut Self;
    fn add_panel_system<T: Component, S: States + Copy, M>(
        &mut self,
        states: S,
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

    fn add_scene_system<T: Component, S: States + Copy, M>(
        &mut self,
        states: S,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.add_systems(OnEnter(states), systems)
            .add_systems(OnExit(states), despawn_screen::<T>)
    }

    fn add_panel_system<T: Component, S: States + Copy, M>(
        &mut self,
        states: S,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.add_systems(OnEnter(states), systems)
            .add_systems(OnExit(states), despawn_screen::<T>)
    }
}
