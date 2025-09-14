use bevy::{asset::uuid::Uuid, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Resource, Deserialize, Serialize)]
pub struct CustomResourceServer {
    pub data: Vec<CustomResource>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomResource {
    pub name: String,
    pub uuid: Uuid,
    pub meger_with: Vec<String>,
}
