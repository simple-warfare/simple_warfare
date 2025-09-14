use bevy::prelude::Resource;

#[derive(Debug, Default, Resource, PartialEq, Eq)]
pub enum ModInfoKind {
    #[default]
    Json,
    Toml,
}



impl ModInfoKind {
    pub fn get_name(&self) -> &'static str{
        match self {
            ModInfoKind::Json => "mod_info.json",
            ModInfoKind::Toml => "mod_info.toml",
        }
    }
}