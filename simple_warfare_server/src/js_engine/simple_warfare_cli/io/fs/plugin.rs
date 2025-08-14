use crate::{
    assets::js_file::{section::SectionFile, toml::TomlFile},
    bevy_ext::condition::read_files_has_data,
};
use bevy::{platform::collections::HashMap, prelude::*};

use super::{SwFsRequestEvent, SwFsRequestReceiver};
pub trait ReadFilesMap {
    type K;
    type V;
    fn get_map(&self) -> &HashMap<Self::K, Self::V>;
}

#[derive(Default, Resource)]
pub struct ReadTomlFiles {
    pub map: HashMap<Handle<TomlFile>, Vec<Box<oneshot::Sender<TomlFile>>>>,
}

impl ReadFilesMap for ReadTomlFiles {
    type K = Handle<TomlFile>;
    type V = Vec<Box<oneshot::Sender<TomlFile>>>;
    fn get_map(&self) -> &HashMap<Self::K, Self::V> {
        &self.map
    }
}

#[derive(Default, Resource)]
pub struct ReadSectionFiles {
    pub map: HashMap<Handle<SectionFile>, Vec<Box<oneshot::Sender<SectionFile>>>>,
}
impl ReadFilesMap for ReadSectionFiles {
    type K = Handle<SectionFile>;
    type V = Vec<Box<oneshot::Sender<SectionFile>>>;
    fn get_map(&self) -> &HashMap<Self::K, Self::V> {
        &self.map
    }
}
pub struct SwFsPlugin;

impl Plugin for SwFsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReadTomlFiles>()
            .init_resource::<ReadSectionFiles>()
            .add_systems(
                Update,
                handle_fs_event.run_if(resource_exists::<SwFsRequestReceiver>),
            )
            .add_systems(
                Update,
                check_read_toml_file.run_if(read_files_has_data::<ReadTomlFiles>()),
            )
            .add_systems(
                Update,
                check_read_section_file.run_if(read_files_has_data::<ReadSectionFiles>()),
            );
    }
}

fn handle_fs_event(
    asset_server: Res<AssetServer>,
    toml_files: Res<Assets<TomlFile>>,
    section_files: Res<Assets<SectionFile>>,
    mut read_toml_files: ResMut<ReadTomlFiles>,
    mut read_section_files: ResMut<ReadSectionFiles>,
    sw_fs_request_receiver: ResMut<SwFsRequestReceiver>,
) -> Result {
    if let Ok(event) = sw_fs_request_receiver
        .0
        .lock()
        .expect("lock js Response receiver error in the system `engine_inited`")
        .try_recv()
    {
        match event {
            SwFsRequestEvent::ReadTomlFile {
                file_sender,
                file_path,
            } => {
                let file_handle = asset_server.load(file_path);
                if asset_server.is_loaded(file_handle.id()) {
                    file_sender.send(toml_files.get(file_handle.id()).cloned().unwrap())?;
                } else {
                    read_toml_files
                        .map
                        .entry(file_handle)
                        .or_default()
                        .push(file_sender);
                }
            }
            SwFsRequestEvent::ReadSectionFile {
                file_sender,
                file_path,
            } => {
                let file_handle = asset_server.load(file_path);
                if asset_server.is_loaded(file_handle.id()) {
                    file_sender.send(section_files.get(file_handle.id()).cloned().unwrap())?;
                } else {
                    read_section_files
                        .map
                        .entry(file_handle)
                        .or_default()
                        .push(file_sender);
                }
            }
        }
    }
    Ok(())
}

fn check_read_toml_file(
    asset_server: Res<AssetServer>,
    mut evnets: EventReader<AssetEvent<TomlFile>>,
    toml_files: Res<Assets<TomlFile>>,
    mut read_toml_files: ResMut<ReadTomlFiles>,
) -> Result {
    for event in evnets.read() {
        if let AssetEvent::LoadedWithDependencies { id } = *event
            && let Some(mut senders) = read_toml_files
                .map
                .remove(&asset_server.get_id_handle(id).unwrap())
        {
            senders.drain(..).for_each(|sender| {
                sender.send(toml_files.get(id).cloned().unwrap()).unwrap();
            });
        }
    }
    Ok(())
}

fn check_read_section_file(
    asset_server: Res<AssetServer>,
    mut evnets: EventReader<AssetEvent<SectionFile>>,
    section_files: Res<Assets<SectionFile>>,
    mut read_section_files: ResMut<ReadSectionFiles>,
) -> Result {
    for event in evnets.read() {
        if let AssetEvent::LoadedWithDependencies { id } = *event
            && let Some(mut senders) = read_section_files
                .map
                .remove(&asset_server.get_id_handle(id).unwrap())
        {
            senders.drain(..).for_each(|sender| {
                sender
                    .send(section_files.get(id).cloned().unwrap())
                    .unwrap();
            });
        }
    }
    Ok(())
}
