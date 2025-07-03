use bevy::{color::palettes::css::*, input::mouse::MouseButtonInput, prelude::*};

use crate::unit::{
    custom_unit::CustomUnit,
    way_point::{ActiveWayPoint, WayPointType},
};

#[derive(Resource, Default, Debug)]
pub struct SelectionState {
    start: Vec2,
    end: Vec2,
    real_start: Vec2,
    real_end: Vec2,
    is_selecting: bool,
}

#[derive(Debug, Component)]
pub struct Selectable;

#[derive(Debug, Component)]
pub struct Selected;

impl SelectionState {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
#[derive(Clone, Default, Resource)]
pub struct MousePosition(Option<Vec2>, Option<Vec2>);

pub fn handle_cursor_move(
    mut cursor_moved_reader: EventReader<CursorMoved>,
    mut selection_state: ResMut<SelectionState>,
    mut mouse_position: ResMut<MousePosition>,
    camera: Single<(&Camera, &GlobalTransform)>,
) -> Result {
    let Some(cursor_moved) = cursor_moved_reader.read().last() else {
        return Ok(());
    };
    let (camera, camera_transform) = *camera;
    mouse_position.0 = Some(cursor_moved.position);
    mouse_position.1 = Some(camera.viewport_to_world_2d(camera_transform, cursor_moved.position)?);
    if selection_state.is_selecting {
        selection_state.end = cursor_moved.position;
    }
    Ok(())
}

pub fn handle_mouse_input(
    mut mouse_input_reader: EventReader<MouseButtonInput>,
    mut selection_state: ResMut<SelectionState>,
    mouse_position: Res<MousePosition>,
) {
    let Some(mouse_pos) = mouse_position.0 else {
        return;
    };

    let Some(mouse_input) = mouse_input_reader.read().last() else {
        return;
    };

    if mouse_input.button != MouseButton::Left {
        return;
    }

    if mouse_input.state.is_pressed() {
        selection_state.is_selecting = true;
        selection_state.start = mouse_pos;
        selection_state.end = mouse_pos;
    } else {
        selection_state.clear();
    }
}

pub fn calculate_world_position_of_selection(
    mut selection_state: ResMut<SelectionState>,
    camera: Single<(&Camera, &GlobalTransform)>,
) -> Result {
    let (camera, camera_transform) = *camera;
    if selection_state.is_selecting {
        selection_state.real_start =
            camera.viewport_to_world_2d(camera_transform, selection_state.start)?;
        selection_state.real_end =
            camera.viewport_to_world_2d(camera_transform, selection_state.end)?;
    }
    Ok(())
}

pub fn draw_selection_box(
    mut gizmos: Gizmos,
    selection_state: Res<SelectionState>,
    camera: Single<(&Camera, &GlobalTransform)>,
) -> Result {
    if selection_state.is_selecting {
        let (camera, camera_transform) = *camera;
        let start = selection_state.start;
        let end = selection_state.end;

        let size = Vec2::new(start.x - end.x, start.y - end.y);
        gizmos.rect_2d(
            Isometry2d::from_translation(
                camera.viewport_to_world_2d(
                    camera_transform,
                    Rect::from_corners(start, end).center(),
                )?,
            ),
            size,
            GREEN,
        );
    }
    Ok(())
}

pub fn updata_selected_unit(
    mut commands: Commands,
    selection_state: Res<SelectionState>,
    units: Query<(Entity, &Transform), With<CustomUnit>>,
) {
    if selection_state.is_selecting {
        let selection_rect =
            calculate_selection_rect(selection_state.real_start, selection_state.real_end);
        for (entity, transform) in units {
            let position = transform.translation.xy();
            if selection_rect.contains(position) {
                commands.entity(entity).insert(Selected);
            } else {
                commands.entity(entity).try_remove::<Selected>();
            }
        }
    }
}

fn calculate_selection_rect(start: Vec2, end: Vec2) -> Rect {
    let min = Vec2::new(start.x.min(end.x), start.y.min(end.y));
    let max = Vec2::new(start.x.max(end.x), start.y.max(end.y));
    Rect::from_corners(min, max)
}

pub fn draw_selected_unit(mut gizmos: Gizmos, selected_units: Query<&Transform, With<Selected>>) {
    for transform in selected_units {
        gizmos.rect_2d(
            Isometry2d::from_translation(transform.translation.xy()),
            Vec2::new(60., 60.),
            GREEN,
        );
    }
}

pub fn test_move(
    mut commands: Commands,
    mut mouse_input_reader: EventReader<MouseButtonInput>,
    selected_units: Query<Entity, With<Selected>>,
    mouse_position: Res<MousePosition>,
) {
    let Some(mouse_world_pos) = mouse_position.1 else {
        return;
    };
    if selected_units.is_empty() {
        return;
    }
    let Some(mouse_input) = mouse_input_reader.read().last() else {
        return;
    };

    if mouse_input.button == MouseButton::Right && mouse_input.state.is_pressed() {
        for entity in selected_units {
            commands
                .entity(entity)
                .insert(ActiveWayPoint(WayPointType::Move(mouse_world_pos)));
        }
    }
}

pub fn test_handle_active_way_point_move(
    mut gizmos: Gizmos,
    units: Query<(&ActiveWayPoint, &Transform)>,
) {
    for (active_way_point, transform) in units {
        match active_way_point.0 {
            WayPointType::Move(target) => {
                gizmos.line_2d(transform.translation.xy(), target, RED);
            }
        }
    }
}
