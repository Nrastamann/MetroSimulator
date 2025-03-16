use bevy::{prelude::*, window::PrimaryWindow};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>();
        app.add_systems(Update, update_cursor_position);
    }
}

#[derive(Default, Resource, Clone)]
pub struct CursorPosition(pub Vec2);

fn update_cursor_position(
    window: Single<&Window, With<PrimaryWindow>>, 
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut position: ResMut<CursorPosition>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else { return };
    let Some(cursor) = window.cursor_position() else { return };
    if let Ok(mouse_position) = camera.viewport_to_world_2d(camera_transform, cursor) {
        position.0 = mouse_position;
    }
}