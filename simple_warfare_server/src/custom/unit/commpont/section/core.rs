use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Hp(i32);

#[derive(Debug, Deref, Component, Clone, DerefMut, PartialEq, Eq)]
pub struct Name(String);

#[derive(Debug, Deref, Component, Clone, DerefMut, PartialEq, Eq)]
pub struct MaxHp(i32);

#[derive(Debug, Deref, Component, Clone, DerefMut, PartialEq, Eq)]
pub struct Mass(u32);

#[derive(Debug, Deref, Component, Clone, DerefMut, PartialEq)]
pub struct BuildSpeed(f32);

#[derive(Debug, Deref, Component, Clone, DerefMut, PartialEq)]
pub struct Radius(f32);

#[derive(Debug, Deref, Component, Clone, DerefMut, PartialEq)]
pub struct EnablePhysics(f32);
