use bevy::{platform::collections::HashMap, prelude::*};

pub trait ReadFilesMap {
    type K;
    type V;
    fn get_map(&self) -> &HashMap<Self::K, Self::V>;
}
pub fn read_files_has_data<R>(read_file: Res<R>) -> bool
where
    R: Resource + ReadFilesMap,
{
    !read_file.get_map().is_empty()
}
