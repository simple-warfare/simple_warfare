use crate::{bevy_ext::prelude::*, define_components};
use bevy::prelude::*;
use simple_warfare_server_macros::TryFromAndIntoJs;

define_components! {
    #[derive(Debug, Component, Clone, Copy, Deref, DerefMut, PartialEq, Eq, TryFromAndIntoJs)]
    Hp|MaxHp|Mass(i32)
}

define_components! {
    #[derive(Debug, Component, Clone, Copy, Deref, DerefMut, PartialEq, TryFromAndIntoJs)]
    #[boa(from_js_with = "f32_try_from_js", into_js_with = "f32_try_into_js")]
    BuildSpeed|Radius|MaxMoveSpeed|MoveAcceleration|MoveDeceleration|
    ReversePercentage|MaxTurnSpeed|TurnAcceleration|TurnDeceleration(f32)
}

define_components! {
    #[derive(Debug, Component, Clone, Copy, Deref, DerefMut, PartialEq,Eq, TryFromAndIntoJs)]
    EnablePhysics(bool)
}

define_components! {
    #[derive(Debug, Component, Clone, Deref, DerefMut, PartialEq,Eq, TryFromAndIntoJs)]
    Name|MovementType(String)
}
