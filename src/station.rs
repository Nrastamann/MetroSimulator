use bevy::{prelude::*, window::PrimaryWindow};

pub struct StationPlugin;

impl Plugin for StationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hover_select);
    }
}

#[derive(Component, Copy, Clone)]
pub struct Station {
    pub position: Vec2,
}

fn hover_select(
    mut stations: Query<&mut Transform, With<Station>>,
    window: Single<&Window, With<PrimaryWindow>>, 
    q_camera: Query<(&Camera, &GlobalTransform)>
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else { return };
    let Some(cursor) = window.cursor_position() else { return };
    let Ok(mouse_position) = camera.viewport_to_world_2d(camera_transform, cursor) else { return };

    for mut station_transform in stations.iter_mut() {
        if station_transform.translation.truncate().distance(mouse_position) < 25. {
            station_transform.scale = Vec3::splat(1.5);
        }
        else {
            station_transform.scale = Vec3::splat(1.0);
        }
    }
}