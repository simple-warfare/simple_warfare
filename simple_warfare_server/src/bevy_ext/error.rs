use bevy::ecs::error::BevyError;

#[derive(Debug, Clone)]
pub enum CommonBevyError<'a> {
    ParentPathNotFound(&'a str),
}

impl<'a> Into<BevyError> for CommonBevyError<'a>{
    fn into(self) -> BevyError {
        match self {
            CommonBevyError::ParentPathNotFound(path) => {
                BevyError::from(format!("Could not get `{}`'s parent path", path))
            }
        }
    }
}
