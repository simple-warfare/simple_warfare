use crate::{
    custom::unit::{
        physics::EnablePhysics,
        unit::CustomUnit,
        way_point::{WayPoint, WayPointQueue},
    },
    js_engine::{JsEngineRequestSender, event::JsEngineRequestEvent},
    scenes::SceneState,
    statistics::*,
};
use bevy::{color::palettes::css::*, input::mouse::MouseButtonInput, prelude::*};

pub struct InputSystemPlugin;

impl Plugin for InputSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionState>()
            .init_resource::<MousePosition>()
            .add_systems(
                FixedUpdate,
                (
                    handle_cursor_move,
                    handle_mouse_input,
                    calculate_world_position_of_selection,
                    (update_selected_unit, add_move_way_point),
                    (
                        draw_selection_box,
                        draw_selected_unit,
                        draw_consecutive_way_point_move,
                    ),
                )
                    .chain()
                    .run_if(in_state(SceneState::GameScene)),
            );
    }
}

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
    mouse_position.windows = Some(cursor_moved.position);
    mouse_position.world =
        Some(camera.viewport_to_world_2d(camera_transform, cursor_moved.position)?);
    if selection_state.is_selecting {
        selection_state.end = cursor_moved.position;
    }
    Ok(())
}

pub fn handle_mouse_input(
    mut mouse_input_reader: EventReader<MouseButtonInput>,
    mut selection_state: ResMut<SelectionState>,
    mouse_position: Res<MousePosition>,
    mut mouse_state: ResMut<NextState<MouseState>>,
) {
    let Some(mouse_pos) = mouse_position.windows else {
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
        mouse_state.set(MouseState::Selected);
        selection_state.start = mouse_pos;
        selection_state.end = mouse_pos;
    } else {
        mouse_state.set(MouseState::Nothing);
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

pub fn draw_selection_box(mut gizmos: Gizmos, selection_state: Res<SelectionState>) -> Result {
    if selection_state.is_selecting {
        let start = selection_state.real_start;
        let end = selection_state.real_end;
        let size = Vec2::new(start.x - end.x, start.y - end.y);
        gizmos.rect_2d(
            Isometry2d::from_translation(Rect::from_corners(start, end).center()),
            size,
            GREEN,
        );
    }
    Ok(())
}

pub fn update_selected_unit(
    mut commands: Commands,
    selection_state: Res<SelectionState>,
    units: Query<(Entity, &Transform), With<CustomUnit>>,
    js_engine_request_sender: Res<JsEngineRequestSender>,
) -> Result {
    if selection_state.is_selecting {
        let selection_rect =
            calculate_selection_rect(selection_state.real_start, selection_state.real_end);
        for (entity, transform) in units {
            let position = transform.translation.xy();
            if selection_rect.contains(position) {
                commands.entity(entity).insert(Selected);
                js_engine_request_sender
                    .0
                    .send(JsEngineRequestEvent::SelectedSignalEmit)?;
            } else {
                commands.entity(entity).try_remove::<Selected>();
            }
        }
    }
    Ok(())
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

pub fn add_move_way_point(
    mut mouse_input_reader: EventReader<MouseButtonInput>,
    selected_units: Query<&mut WayPointQueue, With<Selected>>,
    mouse_position: Res<MousePosition>,
) {
    let Some(mouse_world_pos) = mouse_position.world else {
        return;
    };
    if selected_units.is_empty() {
        return;
    }
    let Some(mouse_input) = mouse_input_reader.read().last() else {
        return;
    };

    if mouse_input.button == MouseButton::Right && mouse_input.state.is_pressed() {
        for mut quene in selected_units {
            quene.data.push_back(WayPoint::Move(mouse_world_pos));
        }
    }
}
pub fn draw_consecutive_way_point_move(
    mut gizmos: Gizmos,
    units: Query<(&WayPointQueue, &Transform), With<EnablePhysics>>,
) {
    for (way_queue, transform) in units {
        let self_translation = WayPoint::Move(transform.translation.xy());
        let waypoints = std::iter::once(&self_translation)
            .chain(way_queue.data.iter())
            .collect::<Vec<_>>();

        for window in waypoints.windows(2) {
            if let [WayPoint::Move(start), WayPoint::Move(target)] = window {
                gizmos.line_2d(*start, *target, RED);
            }
        }
    }
}
//fn draw_consecutive_move_line(mut gizmos: Gizmos, units: Query<(&ActiveWayPoint, &Transform)>) {
//
//}
/*


pub fn test_move(
    mut commands: Commands,
    mut mouse_input_reader: EventReader<MouseButtonInput>,
    selected_units: Query<Entity, With<Selected>>,
    mouse_position: Res<MousePosition>,
) {
    let Some(mouse_world_pos) = mouse_position.world else {
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


*/
