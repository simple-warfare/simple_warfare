use crate::{
    custom::unit::{
        physics::EnablePhysics,
        unit::{CustomUnit, JsUnit, JsUnitData},
        way_point::{WayPoint, WayPointQueue},
    },
    js_engine::{event::JsEngineRequestEvent, JsEngineRequestSender},
    scenes::SceneState,
    statistics::*,
};
use bevy::{color::palettes::css::*, ecs::spawn::SpawnWith, prelude::*};
use bevy_enhanced_input::prelude::*;
pub struct UnitInputSystemPlugin;

#[derive(Component)]
pub struct UnitInputContext;

#[derive(InputAction)]
#[action_output(bool)]
pub struct SelectUnitAction;

impl SelectUnitAction {
    const KEY: MouseButton = MouseButton::Left;
}

#[derive(InputAction)]
#[action_output(bool)]
pub struct AddMoveWayPointAction;

impl AddMoveWayPointAction {
    const KEY: MouseButton = MouseButton::Right;
}

impl Plugin for UnitInputSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<UnitInputContext>()
            .init_resource::<SelectionState>()
            .init_resource::<MousePosition>()
            .add_systems(Startup, setup)
            .add_systems(Update, handle_cursor_move)
            .add_observer(select_unit_fired)
            .add_observer(select_unit_end)
            .add_observer(add_move_way_point)
            .add_systems(
                FixedUpdate,
                (
                    handle_cursor_move,
                    calculate_viewport_to_world_2d,
                    update_selected_unit,
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

pub fn setup(mut commands: Commands) {
    commands.spawn((
        UnitInputContext,
        Actions::<UnitInputContext>::spawn(SpawnWith(|context: &mut ActionSpawner<_>| {
            context.spawn((
                Action::<SelectUnitAction>::new(),
                Down::new(0.05),
                bindings![SelectUnitAction::KEY],
            ));
            context.spawn((
                Action::<AddMoveWayPointAction>::new(),
                Press::default(),
                bindings![AddMoveWayPointAction::KEY],
            ));
        })),
    ));
}

pub fn select_unit_fired(
    _trigger: Trigger<Fired<SelectUnitAction>>,
    mouse_position: Res<MousePosition>,
    mut selection_state: ResMut<SelectionState>,
) {
    let Some(mouse_pos) = mouse_position.viewport else {
        return;
    };
    if selection_state.is_selecting {
        selection_state.viewport_end = mouse_pos;
    } else {
        selection_state.is_selecting = true;
        selection_state.viewport_start = mouse_pos;
        selection_state.viewport_end = mouse_pos;
    }
}

pub fn select_unit_end(
    _trigger: Trigger<Completed<SelectUnitAction>>,
    mut selection_state: ResMut<SelectionState>,
) {
    selection_state.clear();
}

pub fn handle_cursor_move(
    mut cursor_moved_reader: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let Some(cursor_moved) = cursor_moved_reader.read().last() else {
        return;
    };
    mouse_position.viewport = Some(cursor_moved.position);
}

pub fn calculate_viewport_to_world_2d(
    mut selection_state: ResMut<SelectionState>,
    mut mouse_position: ResMut<MousePosition>,
    camera: Single<(&Camera, &GlobalTransform)>,
) -> Result {
    let (camera, camera_transform) = *camera;
    if selection_state.is_selecting {
        selection_state.world_2d_start =
            camera.viewport_to_world_2d(camera_transform, selection_state.viewport_start)?;
        selection_state.world_2d_end =
            camera.viewport_to_world_2d(camera_transform, selection_state.viewport_end)?;
    }

    let Some(mouse_pos) = mouse_position.viewport else {
        return Ok(());
    };
    mouse_position.world_2d = Some(camera.viewport_to_world_2d(camera_transform, mouse_pos)?);
    Ok(())
}

pub fn draw_selection_box(mut gizmos: Gizmos, selection_state: Res<SelectionState>) -> Result {
    if selection_state.is_selecting {
        let start = selection_state.world_2d_start;
        let end = selection_state.world_2d_end;
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
            calculate_selection_rect(selection_state.world_2d_start, selection_state.world_2d_end);
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
    _trigger: Trigger<Fired<AddMoveWayPointAction>>,
    selected_units: Query<(&JsUnitData, &mut WayPointQueue), With<Selected>>,
    mouse_position: Res<MousePosition>,
    js_engine_request_sender: Res<JsEngineRequestSender>,
) -> Result {
    let Some(mouse_pos) = mouse_position.world_2d else {
        return Ok(());
    };
    for (js_unit_data, mut quene) in selected_units {
        let way_point = WayPoint::Move(mouse_pos);
        // quene.data.push_back(way_point.clone());
        // js_engine_request_sender
        //     .0
        //     .send(JsEngineRequestEvent::new_way_point_signal(
        //         way_point,
        //         js_unit_data.new_way_point_entity,
        //     ))?;
    }

    Ok(())
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
